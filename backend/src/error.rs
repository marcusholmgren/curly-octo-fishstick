use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::Error as DieselError;
use diesel::ConnectionError;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    DatabaseError(DieselError),
    ConnectionError(ConnectionError),
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
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => ApiError::NotFound,
            _ => ApiError::DatabaseError(e),
        }
    }
}

impl From<ConnectionError> for ApiError {
    fn from(e: ConnectionError) -> Self {
        ApiError::ConnectionError(e)
    }
}