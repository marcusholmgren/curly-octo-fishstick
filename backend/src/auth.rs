// backend/src/auth.rs
// This file handles JWT-based authentication and token validation.
// It fetches OIDC configuration and JWKS from an identity provider to validate tokens.
// RELEVANT FILES: backend/src/main.rs, backend/src/handlers.rs

use actix_web::{dev::Payload, web, Error as ActixWebError, FromRequest, HttpRequest};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::RwLock;

/// Represents the possible errors that can occur during authentication.
#[derive(Debug, Error)]
pub enum AuthError {
    /// Error for a missing or malformed Authorization header.
    #[error("Missing or malformed Authorization header")]
    MissingToken,
    /// Error for an invalid token, wrapping the underlying JWT error.
    #[error("The token provided is invalid")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),
    /// Error when a public key for the given token KID is not found.
    #[error("Could not find a public key for the given token KID: {0}")]
    KeyNotFound(String),
    /// Error for network issues while fetching OIDC config or JWKS.
    #[error("Network error while fetching OIDC config or JWKS: {0}")]
    NetworkError(#[from] reqwest::Error),
    /// Error when a valid RSA public key cannot be constructed from JWK components.
    #[error("Could not construct a valid RSA public key from JWK components")]
    KeyConstructionError,
}

impl actix_web::ResponseError for AuthError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AuthError::MissingToken | AuthError::InvalidToken(_) | AuthError::KeyNotFound(_) => {
                actix_web::http::StatusCode::UNAUTHORIZED
            }
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let mut res = actix_web::HttpResponse::build(self.status_code()).body(self.to_string());
        if let AuthError::MissingToken | AuthError::InvalidToken(_) = self {
            res.headers_mut().insert(
                actix_web::http::header::WWW_AUTHENTICATE,
                actix_web::http::header::HeaderValue::from_static("Bearer error=\"invalid_token\""),
            );
        }
        res
    }
}

/// Represents the OIDC configuration fetched from the identity provider.
#[derive(Debug, Deserialize, Clone)]
pub struct OidcConfig {
    /// The URI for the JSON Web Key Set (JWKS).
    pub jwks_uri: String,
    /// The issuer of the tokens.
    pub issuer: String,
}

/// Represents a single JSON Web Key (JWK).
#[derive(Debug, Deserialize, Clone)]
pub struct JsonWebKey {
    /// The Key ID.
    pub kid: String,
    /// The algorithm used for the key (e.g., "RS256").
    pub alg: String,
    /// The modulus for an RSA public key.
    pub n: String,
    /// The exponent for an RSA public key.
    pub e: String,
}

/// Represents a set of JSON Web Keys (JWKS).
#[derive(Debug, Deserialize, Clone)]
pub struct Jwks {
    /// A vector of `JsonWebKey`s.
    pub keys: Vec<JsonWebKey>,
}

/// Represents the claims extracted from a validated JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// The subject identifier.
    pub sub: Option<String>,
    /// The preferred username of the user.
    pub preferred_username: String,
    /// The email address of the user.
    pub email: Option<String>,
    /// The audience for which the token is intended.
    pub aud: String,
    /// The issuer of the token.
    pub iss: String,
    /// The expiration time of the token (as a Unix timestamp).
    pub exp: usize,
}

/// A simple cache for OIDC configuration and JWKS.
#[derive(Default)]
struct Cache {
    /// The cached OIDC configuration and the time it was cached.
    well_known_config: Option<(OidcConfig, Instant)>,
    /// The cached JWKS and the time it was cached.
    jwks: Option<(Jwks, Instant)>,
}

/// A service for validating JWTs using OIDC and JWKS.
///
/// It includes a caching mechanism to avoid fetching the configuration and keys on every request.
pub struct TokenValidator {
    client: Client,
    idp_url: String,
    audience: String,
    cache: RwLock<Cache>,
    cache_ttl: Duration,
}

impl TokenValidator {
    /// Creates a new `TokenValidator`.
    ///
    /// # Arguments
    ///
    /// * `idp_url` - The base URL of the identity provider.
    /// * `audience` - The expected audience of the JWTs.
    ///
    /// # Returns
    ///
    /// * A new `TokenValidator` instance.
    pub fn new(idp_url: &str, audience: &str) -> Self {
        Self {
            client: Client::new(),
            idp_url: idp_url.to_string(),
            audience: audience.to_string(),
            cache: RwLock::new(Cache::default()),
            cache_ttl: Duration::from_secs(300), // 5 minutes cache
        }
    }

    /// Fetches the OIDC well-known configuration, using a cache to avoid repeated requests.
    ///
    /// # Returns
    ///
    /// * `Ok(OidcConfig)` if the configuration is fetched successfully.
    /// * `Err(AuthError)` if there is an error.
    async fn get_well_known_config(&self) -> Result<OidcConfig, AuthError> {
        // Check read-only cache first
        let cached_config = self.cache.read().await.well_known_config.clone();
        if let Some((config, timestamp)) = cached_config {
            if timestamp.elapsed() < self.cache_ttl {
                return Ok(config);
            }
        }

        // If not in cache or expired, fetch
        log::info!("Fetching new OIDC well-known configuration...");
        let url = format!("{}/.well-known/openid-configuration", self.idp_url);
        let config: OidcConfig = self.client.get(&url).send().await?.json().await?;

        // Acquire write lock to update cache
        let mut cache = self.cache.write().await;
        cache.well_known_config = Some((config.clone(), Instant::now()));

        Ok(config)
    }

    /// Fetches the JSON Web Key Set (JWKS), using a cache.
    ///
    /// # Returns
    ///
    /// * `Ok(Jwks)` if the JWKS is fetched successfully.
    /// * `Err(AuthError)` if there is an error.
    async fn get_jwks(&self) -> Result<Jwks, AuthError> {
        // Check read-only cache first
        let cached_jwks = self.cache.read().await.jwks.clone();
        if let Some((jwks, timestamp)) = cached_jwks {
            if timestamp.elapsed() < self.cache_ttl {
                return Ok(jwks);
            }
        }

        // If not in cache or expired, fetch config
        let config = self.get_well_known_config().await?;

        // Now fetch JWKS
        log::info!("Fetching new JWKS...");
        let jwks: Jwks = self
            .client
            .get(&config.jwks_uri)
            .send()
            .await?
            .json()
            .await?;

        // Acquire write lock to update cache
        let mut cache = self.cache.write().await;
        cache.jwks = Some((jwks.clone(), Instant::now()));

        Ok(jwks)
    }

    /// Gets the decoding key for a given Key ID (KID).
    ///
    /// # Arguments
    ///
    /// * `kid` - The Key ID from the JWT header.
    ///
    /// # Returns
    ///
    /// * `Ok(DecodingKey)` if the key is found and constructed successfully.
    /// * `Err(AuthError)` if the key is not found or cannot be constructed.
    async fn get_decoding_key(&self, kid: &str) -> Result<DecodingKey, AuthError> {
        let jwks = self.get_jwks().await?;
        let jwk = jwks
            .keys
            .iter()
            .find(|key| key.kid == kid)
            .ok_or_else(|| AuthError::KeyNotFound(kid.to_string()))?;

        // Construct the RSA DecodingKey from the public key components (n, e)
        DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
            .map_err(|_| AuthError::KeyConstructionError)
    }

    /// Decodes and validates a JWT.
    ///
    /// # Arguments
    ///
    /// * `token` - The JWT string to decode.
    ///
    /// # Returns
    ///
    /// * `Ok(Claims)` if the token is valid.
    /// * `Err(AuthError)` if the token is invalid or another error occurs.
    pub async fn decode_token(&self, token: &str) -> Result<Claims, AuthError> {
        let header = decode_header(token)?;
        let kid = header
            .kid
            .ok_or_else(|| AuthError::KeyNotFound("No KID in header".to_string()))?;

        let decoding_key = self.get_decoding_key(&kid).await?;

        let mut validation = Validation::new(header.alg);
        validation.set_audience(&[self.audience.clone()]);

        let config = self.get_well_known_config().await?;
        validation.set_issuer(&[config.issuer]);

        let decoded_token = decode::<Claims>(token, &decoding_key, &validation)?;
        Ok(decoded_token.claims)
    }
}

/// Implements `FromRequest` for `Claims`, allowing it to be used as a request guard.
///
/// This extracts the token from the `Authorization` header, validates it, and extracts the claims.
impl FromRequest for Claims {
    type Error = ActixWebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let validator = req
                .app_data::<web::Data<TokenValidator>>()
                .ok_or(AuthError::KeyConstructionError)?;

            let token = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "))
                .ok_or(AuthError::MissingToken)?;

            validator.decode_token(token).await.map_err(|e| {
                log::error!("Token validation error: {:?}", e);
                e.into()
            })
        })
    }
}
