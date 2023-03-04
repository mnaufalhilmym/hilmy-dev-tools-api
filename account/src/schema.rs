// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
