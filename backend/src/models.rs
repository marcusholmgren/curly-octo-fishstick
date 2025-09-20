use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable)]
#[diesel(table_name = crate::schema::contacts)]
pub struct Contact {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::contacts)]
pub struct NewContact {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
}
