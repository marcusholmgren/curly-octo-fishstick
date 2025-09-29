// backend/src/error.rs
// This file defines the custom error types used throughout the API.
// It provides a unified way to handle different kinds of errors and convert them into appropriate HTTP responses.
// RELEVANT FILES: backend/src/handlers.rs, backend/src/main.rs

use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::Error as DieselError;
use diesel::ConnectionError;
use std::fmt;

/// Represents the possible errors that can occur in the API.
#[derive(Debug)]
pub enum ApiError {
    /// A database-related error, wrapping `diesel::result::Error`.
    DatabaseError(DieselError),
    /// A connection error, wrapping `diesel::ConnectionError`.
    ConnectionError(ConnectionError),
    /// An error indicating that a requested resource was not found.
    NotFound,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::DatabaseError(e) => write!(f, "Database error: {}", e),
            ApiError::ConnectionError(e) => write!(f, "Connection error: {}", e),
            ApiError::NotFound => write!(f, "Not Found"),
        }
    }
}

impl ResponseError for ApiError {
    /// Maps an `ApiError` to an `HttpResponse`.
    ///
    /// # Returns
    ///
    /// * An `HttpResponse` with an appropriate status code and message.
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::DatabaseError(_) | ApiError::ConnectionError(_) => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ApiError::NotFound => HttpResponse::NotFound().json("Not Found"),
        }
    }
}

impl From<DieselError> for ApiError {
    /// Converts a `diesel::result::Error` into an `ApiError`.
    ///
    /// # Arguments
    ///
    /// * `e` - The `DieselError` to convert.
    ///
    /// # Returns
    ///
    /// * The corresponding `ApiError`.
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => ApiError::NotFound,
            _ => ApiError::DatabaseError(e),
        }
    }
}

impl From<ConnectionError> for ApiError {
    /// Converts a `diesel::ConnectionError` into an `ApiError`.
    ///
    /// # Arguments
    ///
    /// * `e` - The `ConnectionError` to convert.
    ///
    /// # Returns
    ///
    /// * The corresponding `ApiError`.
    fn from(e: ConnectionError) -> Self {
        ApiError::ConnectionError(e)
    }
}