use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use uuid::Uuid;

use crate::schema;

use super::AccountRole;

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::account)]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub role: AccountRole,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
