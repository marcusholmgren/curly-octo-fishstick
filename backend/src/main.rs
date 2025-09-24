use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;

mod auth;
pub mod error;
pub mod handlers;
pub mod models;
pub mod schema;

use crate::auth::TokenValidator;
use crate::error::ApiError;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migrations(
    conn: &mut impl MigrationHarness<diesel::sqlite::Sqlite>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

// Helper function to establish a database connection
fn establish_connection() -> Result<SqliteConnection, ApiError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).map_err(|e| ApiError::from(e))
}

use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut conn = establish_connection().expect("Failed to connect to database");
    run_migrations(&mut conn).expect("Failed to run database migrations");

    if dotenvy::dotenv().is_err() {
        log::warn!(".env file not found, relying on environment variables.");
    }
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let idp_url = std::env::var("IDP_URL")
        .expect("IDP_URL environment variable must be set, e.g., in a .env file.");
    let idp_audience = std::env::var("IDP_AUDIENCE")
        .expect("IDP_AUDIENCE environment variable must be set, e.g., in a .env file.");

    let validator = web::Data::new(TokenValidator::new(&idp_url, &idp_audience));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // Add your frontend origins
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .app_data(validator.clone())
            .service(
                web::scope("/api")
                    .service(handlers::create_contact)
                    .service(handlers::read_contacts)
                    .service(handlers::read_contact)
                    .service(handlers::update_contact)
                    .service(handlers::delete_contact),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
