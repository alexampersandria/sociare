// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        password -> Varchar,
        created_at -> Int8,
    }
}
