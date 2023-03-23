use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use uuid::Uuid;

use crate::schema;

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::apprepo)]
pub struct Apprepo {
    pub id: Uuid,
    pub name: String,
    pub icon: String,
    pub link: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = schema::apprepo)]
pub struct ApprepoChangeSet {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub link: Option<String>,
}
