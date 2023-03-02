use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use uuid::Uuid;

use crate::schema;

use super::ServiceAddressStatus;

#[cfg_attr(debug_assertions, derive(Debug))]
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
