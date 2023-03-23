use std::io::Write;

use diesel::{
    deserialize::{self, FromSql},
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};

use crate::schema;

#[derive(Debug, diesel::AsExpression, diesel::FromSqlRow)]
#[diesel(sql_type = schema::sql_types::AccountRole)]
pub enum AccountRole {
    User,
    Admin,
}

impl ToSql<schema::sql_types::AccountRole, Pg> for AccountRole {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            AccountRole::User => out.write_all(b"user")?,
            AccountRole::Admin => out.write_all(b"admin")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<schema::sql_types::AccountRole, Pg> for AccountRole {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"user" => Ok(AccountRole::User),
            b"admin" => Ok(AccountRole::Admin),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl AccountRole {
    pub fn to_grpc_enum(&self) -> i32 {
        match self {
            AccountRole::User => 0,
            AccountRole::Admin => 1,
        }
    }
}
