use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use uuid::Uuid;

use crate::schema;

use super::ServiceAddressStatus;

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::service_address)]
pub struct ServiceAddress {
    pub id: Uuid,
    pub service_id: Uuid,
    pub address: String,
    pub status: ServiceAddressStatus,
    pub last_used_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = schema::service_address)]
pub struct ServiceAddressChangeSet {
    pub service_id: Option<Uuid>,
    pub address: Option<String>,
    pub status: Option<ServiceAddressStatus>,
    pub last_used_at: Option<NaiveDateTime>,
}
