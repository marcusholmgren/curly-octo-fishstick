use actix_web::{App, HttpServer, web};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use std::env;

pub mod error;
pub mod handlers;
pub mod models;
pub mod schema;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut conn = establish_connection().expect("Failed to connect to database");
    run_migrations(&mut conn).expect("Failed to run database migrations");

    HttpServer::new(move || {
        App::new().service(
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
