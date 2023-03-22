// @generated automatically by Diesel CLI.

diesel::table! {
    link (id) {
        id -> Uuid,
        title -> Text,
        short_url -> Text,
        long_url -> Text,
        visits -> Int4,
        created_by_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
