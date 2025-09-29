// backend/src/handlers.rs
// This file contains the HTTP handlers for the API endpoints.
// It defines the logic for creating, reading, updating, and deleting contacts.
// RELEVANT FILES: backend/src/main.rs, backend/src/models.rs, backend/src/error.rs

use crate::auth::Claims;
use crate::error::ApiError;
use crate::establish_connection;
use crate::models::{Contact, NewContact};
use actix_web::{delete, get, post, put, web, HttpResponse};
use diesel::prelude::*;

/// Handles the creation of a new contact.
///
/// This endpoint is protected and requires a valid JWT.
///
/// # Arguments
///
/// * `_claims` - The claims extracted from the JWT, used for authentication.
/// * `contact` - The new contact data from the request body.
///
/// # Returns
///
/// * `Ok(HttpResponse)` with a success message if the contact is created.
/// * `Err(ApiError)` if there is a database error.
#[post("/contacts")]
pub async fn create_contact(
    _claims: Claims,
    contact: web::Json<NewContact>,
) -> Result<HttpResponse, ApiError> {
    let mut conn = establish_connection()?;

    diesel::insert_into(crate::schema::contacts::table)
        .values(&contact.into_inner())
        .execute(&mut conn)?;

    Ok(HttpResponse::Ok().body("Contact created successfully"))
}

/// Handles reading all contacts from the database.
///
/// This endpoint is protected and requires a valid JWT.
///
/// # Arguments
///
/// * `_claims` - The claims extracted from the JWT, used for authentication.
///
/// # Returns
///
/// * `Ok(HttpResponse)` with a JSON array of contacts.
/// * `Err(ApiError)` if there is a database error.
#[get("/contacts")]
pub async fn read_contacts(_claims: Claims) -> Result<HttpResponse, ApiError> {
    let mut conn = establish_connection()?;

    let contacts = crate::schema::contacts::table
        .order((
            crate::schema::contacts::last_name.asc(),
            crate::schema::contacts::first_name.asc(),
        ))
        .load::<Contact>(&mut conn)?;

    Ok(HttpResponse::Ok().json(contacts))
}

/// Handles reading a specific contact by its ID.
///
/// This endpoint is protected and requires a valid JWT.
///
/// # Arguments
///
/// * `_claims` - The claims extracted from the JWT, used for authentication.
/// * `id` - The ID of the contact to read, from the URL path.
///
/// # Returns
///
/// * `Ok(HttpResponse)` with the JSON data for the contact.
/// * `Err(ApiError)` if the contact is not found or there is a database error.
#[get("/contacts/{id}")]
pub async fn read_contact(
    _claims: Claims,
    id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let mut conn = establish_connection()?;

    let contact = crate::schema::contacts::table
        .find(id.into_inner())
        .first::<Contact>(&mut conn)?;

    Ok(HttpResponse::Ok().json(contact))
}

/// Handles updating an existing contact by its ID.
///
/// This endpoint is protected and requires a valid JWT.
///
/// # Arguments
///
/// * `_claims` - The claims extracted from the JWT, used for authentication.
/// * `id` - The ID of the contact to update, from the URL path.
/// * `contact` - The updated contact data from the request body.
///
/// # Returns
///
/// * `Ok(HttpResponse)` with a success message if the contact is updated.
/// * `Err(ApiError)` if the contact is not found or there is a database error.
#[put("/contacts/{id}")]
pub async fn update_contact(
    _claims: Claims,
    id: web::Path<i32>,
    contact: web::Json<NewContact>,
) -> Result<HttpResponse, ApiError> {
    let mut conn = establish_connection()?;

    diesel::update(crate::schema::contacts::table.find(id.into_inner()))
        .set(contact.into_inner())
        .execute(&mut conn)?;

    Ok(HttpResponse::Ok().body("Contact updated successfully"))
}

/// Handles deleting a contact by its ID.
///
/// This endpoint is protected and requires a valid JWT.
///
/// # Arguments
///
/// * `_claims` - The claims extracted from the JWT, used for authentication.
/// * `id` - The ID of the contact to delete, from the URL path.
///
/// # Returns
///
/// * `Ok(HttpResponse)` with a success message if the contact is deleted.
/// * `Err(ApiError)` if the contact is not found or there is a database error.
#[delete("/contacts/{id}")]
pub async fn delete_contact(_claims: Claims, id: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let mut conn = establish_connection()?;

    diesel::delete(crate::schema::contacts::table.find(id.into_inner())).execute(&mut conn)?;

    Ok(HttpResponse::Ok().body("Contact deleted successfully"))
}
