// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        phone_number -> Text,
    }
}
