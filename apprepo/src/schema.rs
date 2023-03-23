// @generated automatically by Diesel CLI.

diesel::table! {
    apprepo (id) {
        id -> Uuid,
        name -> Text,
        icon -> Text,
        link -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
