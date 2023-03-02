use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use uuid::Uuid;

use crate::schema;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::service_info)]
pub struct ServiceInfo {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
