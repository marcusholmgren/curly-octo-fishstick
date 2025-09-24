use crate::auth::Claims;
use crate::error::ApiError;
use crate::establish_connection;
use crate::models::{Contact, NewContact};
use actix_web::{delete, get, post, put, web, HttpResponse};
use diesel::prelude::*;

// Create a new contact
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

// Read all contacts
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

// Read a specific contact by ID
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

// Update a contact by ID
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

// Delete a contact by ID
#[delete("/contacts/{id}")]
pub async fn delete_contact(_claims: Claims, id: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let mut conn = establish_connection()?;

    diesel::delete(crate::schema::contacts::table.find(id.into_inner())).execute(&mut conn)?;

    Ok(HttpResponse::Ok().body("Contact deleted successfully"))
}
