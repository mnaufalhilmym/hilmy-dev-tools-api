use std::io::Write;

use diesel::{
    deserialize::{self, FromSql},
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};

use crate::schema;

#[derive(
    Clone, Copy, PartialEq, Eq, Debug, async_graphql::Enum, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = schema::sql_types::ServiceAddressStatus)]
pub enum ServiceAddressStatus {
    Accessible,
    Inaccessible,
}

impl ToSql<schema::sql_types::ServiceAddressStatus, Pg> for ServiceAddressStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ServiceAddressStatus::Accessible => out.write_all(b"accessible")?,
            ServiceAddressStatus::Inaccessible => out.write_all(b"inaccessible")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<schema::sql_types::ServiceAddressStatus, Pg> for ServiceAddressStatus {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"accessible" => Ok(ServiceAddressStatus::Accessible),
            b"inaccessible" => Ok(ServiceAddressStatus::Inaccessible),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
