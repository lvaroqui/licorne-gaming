// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        username -> Varchar,
        password_hash -> Varchar,
    }
}
