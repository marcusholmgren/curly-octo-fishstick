use actix_web::{dev::Payload, web, Error as ActixWebError, FromRequest, HttpRequest};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::RwLock;

// --- Custom Error Types for Clearer Error Handling ---
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Missing or malformed Authorization header")]
    MissingToken,
    #[error("The token provided is invalid")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),
    #[error("Could not find a public key for the given token KID: {0}")]
    KeyNotFound(String),
    #[error("Network error while fetching OIDC config or JWKS: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Could not construct a valid RSA public key from JWK components")]
    KeyConstructionError,
}

// --- Implement ResponseError to map our errors to HTTP responses ---
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

// --- Data Structures for OIDC/JWKS ---
#[derive(Debug, Deserialize, Clone)]
pub struct OidcConfig {
    pub jwks_uri: String,
    pub issuer: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JsonWebKey {
    pub kid: String, // Key ID
    pub alg: String, // Algorithm (e.g., "RS256")
    pub n: String,   // Modulus
    pub e: String,   // Exponent
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwks {
    pub keys: Vec<JsonWebKey>,
}

// --- User claims extracted from the JWT ---
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Option<String>,
    pub preferred_username: String,
    pub email: Option<String>,
    pub aud: String, // Audience
    pub iss: String, // Issuer
    pub exp: usize,  // Expiration time
}

// --- Caching mechanism ---
#[derive(Default)]
struct Cache {
    well_known_config: Option<(OidcConfig, Instant)>,
    jwks: Option<(Jwks, Instant)>,
}

// --- The core service for token validation ---
pub struct TokenValidator {
    client: Client,
    idp_url: String,
    audience: String,
    cache: RwLock<Cache>,
    cache_ttl: Duration,
}

impl TokenValidator {
    pub fn new(idp_url: &str, audience: &str) -> Self {
        Self {
            client: Client::new(),
            idp_url: idp_url.to_string(),
            audience: audience.to_string(),
            cache: RwLock::new(Cache::default()),
            cache_ttl: Duration::from_secs(300), // 5 minutes cache
        }
    }

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

// --- `FromRequest` Implementation ---
// This is the magic that lets us use `claims: Claims` as a handler argument.
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
