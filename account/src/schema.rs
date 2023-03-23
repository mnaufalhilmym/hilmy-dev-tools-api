// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType, diesel::query_builder::QueryId)]
    #[diesel(postgres_type(name = "account_role"))]
    pub struct AccountRole;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AccountRole;

    account (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        role -> AccountRole,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
