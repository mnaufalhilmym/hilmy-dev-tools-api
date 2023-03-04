// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType, diesel::query_builder::QueryId)]
    #[diesel(postgres_type(name = "service_address_status"))]
    pub struct ServiceAddressStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ServiceAddressStatus;

    service_address (id) {
        id -> Uuid,
        service_id -> Uuid,
        address -> Text,
        status -> ServiceAddressStatus,
        last_used_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    service_info (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(service_address -> service_info (service_id));

diesel::allow_tables_to_appear_in_same_query!(
    service_address,
    service_info,
);
