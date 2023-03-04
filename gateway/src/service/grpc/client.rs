use std::{error::Error, time::Duration};

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
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
    let service_info = schema::service_info::table
        .filter(schema::service_info::name.eq(&service_name))
        .first::<model::ServiceInfo>(db_conn)?;
    let service_addresses = schema::service_address::table
        .filter(schema::service_address::status.eq(&model::ServiceAddressStatus::Accessible))
        .filter(schema::service_address::service_id.eq(&service_info.id))
        .order(schema::service_address::address.asc())
        .load::<model::ServiceAddress>(db_conn)?;

    if service_addresses.is_empty() {
        return Err(format!("{} service addresses not found.", service_info.name).into());
    }

    let mut channel: Option<Channel> = None;
    let mut used_service_address_id: Option<Uuid> = None;

    for (index, service_address) in service_addresses.iter().enumerate() {
        channel = match connect(&service_address.address).await {
            Ok(client) => {
                used_service_address_id = Some(service_address.id);
                Some(client)
            }
            Err(_) => {
                diesel::update(schema::service_address::table.find(service_address.id))
                    .set(schema::service_address::status.eq(ServiceAddressStatus::Inaccessible))
                    .execute(db_conn)?;
                if index < service_addresses.len() - 1 {
                    continue;
                }
                None
            }
        };
    }

    if let Some(channel) = channel {
        let _ =
            diesel::update(schema::service_address::table.find(used_service_address_id.unwrap()))
                .set(schema::service_address::last_used_at.eq(diesel::dsl::now))
                .execute(db_conn);

        return Ok(channel);
    }

    Err(format!("Can't get client.").into())
}
