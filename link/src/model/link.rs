use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use uuid::Uuid;

use crate::schema;

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::link)]
pub struct Link {
    pub id: Uuid,
    pub title: String,
    pub short_url: String,
    pub long_url: String,
    pub visits: i32,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = schema::link)]
pub struct LinkChangeSet {
    pub title: Option<String>,
    pub short_url: Option<String>,
    pub long_url: Option<String>,
}
