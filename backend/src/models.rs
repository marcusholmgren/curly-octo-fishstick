// backend/src/models.rs
// This file defines the data structures for the contacts in the database.
// It includes structs for both reading existing contacts and creating new ones.
// RELEVANT FILES: backend/src/handlers.rs, backend/src/schema.rs

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a contact retrieved from the database.
///
/// This struct is used for serialization and deserialization of contact data
/// when reading from the database.
#[derive(Deserialize, Serialize, Queryable)]
#[diesel(table_name = crate::schema::contacts)]
pub struct Contact {
    /// The unique identifier for the contact.
    pub id: i32,
    /// The first name of the contact.
    pub first_name: String,
    /// The last name of the contact.
    pub last_name: String,
    /// The email address of the contact.
    pub email: String,
    /// The phone number of the contact.
    pub phone_number: String,
}

/// Represents a new contact to be inserted into the database.
///
/// This struct is used for deserializing new contact data from requests
/// and for inserting new records into the database. It is also used for updating
/// existing contacts.
#[derive(Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::contacts)]
pub struct NewContact {
    /// The first name of the new contact.
    pub first_name: String,
    /// The last name of the new contact.
    pub last_name: String,
    /// The email address of the new contact.
    pub email: String,
    /// The phone number of the new contact.
    pub phone_number: String,
}
