use std::{error::Error, time::Duration};

use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use tonic::transport::Channel;
use tools_lib_db::pg::connection::DbPooled;
use uuid::Uuid;

use crate::{
    model::{self, ServiceAddressStatus},
    schema,
};

pub async fn connect(service_address: &str) -> Result<Channel, Box<dyn Error + Send + Sync>> {
    match Channel::from_shared(service_address.to_owned())?
        .connect_timeout(Duration::from_millis(500))
        .connect()
        .await
    {
        Ok(channel) => Ok(channel),
        Err(e) => Err(e.into()),
    }
}

pub async fn get(
    db_conn: &mut DbPooled,
    service_name: &str,
) -> Result<Channel, Box<dyn Error + Send + Sync>> {
    let service_addresses = schema::service_address::table
        .filter(schema::service_info::name.eq(&service_name))
        .filter(schema::service_address::status.eq(&model::ServiceAddressStatus::Accessible))
        .left_join(
            schema::service_info::table
                .on(schema::service_address::service_id.eq(schema::service_info::id)),
        )
        .order(schema::service_address::last_used_at.asc())
        .select((
            schema::service_address::id,
            schema::service_address::address,
        ))
        .load::<(Uuid, String)>(db_conn)?;

    if service_addresses.is_empty() {
        return Err(format!("{} service addresses not found.", service_name).into());
    }

    let mut channel: Option<Channel> = None;
    let mut used_service_address_id: Option<Uuid> = None;

    for (index, service_address) in service_addresses.iter().enumerate() {
        channel = match connect(&service_address.1).await {
            Ok(client) => {
                used_service_address_id = Some(service_address.0);
                Some(client)
            }
            Err(_) => {
                diesel::update(schema::service_address::table.find(service_address.0))
                    .set((
                        schema::service_address::status.eq(ServiceAddressStatus::Inaccessible),
                        schema::service_address::updated_at.eq(diesel::dsl::now),
                    ))
                    .execute(db_conn)?;
                if index < service_addresses.len() - 1 {
                    continue;
                }
                None
            }
        };
        if channel.is_some() {
            break;
        }
    }

    if let Some(channel) = channel {
        diesel::update(schema::service_address::table.find(used_service_address_id.unwrap()))
            .set((
                schema::service_address::last_used_at.eq(diesel::dsl::now),
                schema::service_address::updated_at.eq(diesel::dsl::now),
            ))
            .execute(db_conn)?;

        return Ok(channel);
    }

    Err(format!("Can't get the client.").into())
}
