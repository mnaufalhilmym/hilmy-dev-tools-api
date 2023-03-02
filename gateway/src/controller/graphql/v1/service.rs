use async_graphql::{Context, Object, Result};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use tools_db::pg::connection::DbPool;
use uuid::Uuid;

use crate::{
    env::AppMode,
    model::{self, ServiceAddressStatus},
    schema, service,
};

struct ServiceInfo {
    id: Uuid,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[Object]
impl ServiceInfo {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
}

#[derive(Default)]
pub struct ServiceInfoQuery;

#[Object]
impl ServiceInfoQuery {
    async fn services_info<'a>(
        &self,
        ctx: &Context<'a>,
        id: Option<Uuid>,
        name: Option<String>,
    ) -> Result<Vec<ServiceInfo>> {
        let app_mode = ctx.data_unchecked::<AppMode>();
        let db_conn = &mut tools_db::pg::connection::get_connection(
            &app_mode,
            &ctx.data_unchecked::<DbPool>(),
        )?;

        let mut query = schema::service_info::table.into_boxed();
        if let Some(id) = id {
            query = query.filter(schema::service_info::id.eq(id));
        } else if let Some(name) = name {
            query = query.filter(schema::service_info::name.eq(name));
        }

        Ok(query
            .load::<model::ServiceInfo>(db_conn)?
            .iter()
            .map(|service_info| ServiceInfo {
                id: service_info.id.to_owned(),
                name: service_info.name.to_owned(),
                created_at: service_info.created_at.to_owned(),
                updated_at: service_info.created_at.to_owned(),
            })
            .collect())
    }
}

struct ServiceAddress {
    id: Uuid,
    service_id: Uuid,
    address: String,
    status: ServiceAddressStatus,
    last_used_at: NaiveDateTime,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[Object]
impl ServiceAddress {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn service_id(&self) -> &Uuid {
        &self.service_id
    }

    async fn address(&self) -> &str {
        &self.address
    }

    async fn status(&self) -> &ServiceAddressStatus {
        &self.status
    }

    async fn last_used_at(&self) -> &NaiveDateTime {
        &self.last_used_at
    }

    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
}

#[derive(Default)]
pub struct ServiceAddressQuery;

#[Object]
impl ServiceAddressQuery {
    async fn services_address<'a>(
        &self,
        ctx: &Context<'a>,
        id: Option<Uuid>,
        service_id: Option<Uuid>,
    ) -> Result<Vec<ServiceAddress>> {
        let app_mode = ctx.data_unchecked::<AppMode>();
        let db_conn = &mut tools_db::pg::connection::get_connection(
            &app_mode,
            &ctx.data_unchecked::<DbPool>(),
        )?;

        let mut query = schema::service_address::table.into_boxed();
        if let Some(id) = id {
            query = query.filter(schema::service_address::id.eq(id));
        } else if let Some(service_id) = service_id {
            query = query.filter(schema::service_address::service_id.eq(service_id));
        }

        Ok(query
            .load::<model::ServiceAddress>(db_conn)?
            .iter()
            .map(|service_address| ServiceAddress {
                id: service_address.id.to_owned(),
                service_id: service_address.service_id.to_owned(),
                address: service_address.address.to_owned(),
                status: service_address.status.to_owned(),
                last_used_at: service_address.last_used_at.to_owned(),
                created_at: service_address.created_at.to_owned(),
                updated_at: service_address.updated_at.to_owned(),
            })
            .collect())
    }
}

struct Service {
    service_id: Uuid,
    service_addrs_id: Uuid,
    name: String,
    address: String,
}

#[Object]
impl Service {
    async fn service_id(&self) -> &Uuid {
        &self.service_id
    }

    async fn service_addrs_id(&self) -> &Uuid {
        &self.service_addrs_id
    }

    async fn name(&self) -> &String {
        &self.name
    }

    async fn address(&self) -> &String {
        &self.address
    }
}

#[derive(Default)]
pub struct ServiceMutation;

#[Object]
impl ServiceMutation {
    async fn add_service<'a>(
        &self,
        ctx: &Context<'a>,
        service_id: Option<Uuid>,
        name: Option<String>,
        address: String,
    ) -> Result<Service> {
        let app_mode = ctx.data_unchecked::<AppMode>();
        let db_conn = &mut tools_db::pg::connection::get_connection(
            &app_mode,
            &ctx.data_unchecked::<DbPool>(),
        )?;

        let client = service::grpc::client::connect(&address).await;
        if let Err(e) = client {
            return Err(format!("Can't reach {}. Error: {}", address, e).into());
        }

        let service_info = if let Some(service_id) = service_id {
            Some(
                schema::service_info::table
                    .find(service_id)
                    .first::<model::ServiceInfo>(db_conn)?,
            )
        } else if let Some(name) = name {
            Some(
                schema::service_info::table
                    .filter(schema::service_info::name.eq(name))
                    .first::<model::ServiceInfo>(db_conn)?,
            )
        } else {
            return Err("Either service_id or name is required.".into());
        };

        let service_address = diesel::insert_into(schema::service_address::table)
            .values((
                schema::service_address::service_id.eq(&service_info.as_ref().unwrap().id),
                schema::service_address::address.eq(&address),
                schema::service_address::status.eq(ServiceAddressStatus::Accessible),
                schema::service_address::last_used_at.eq(diesel::dsl::now),
            ))
            .get_result::<model::ServiceAddress>(db_conn)?;

        Ok(Service {
            service_addrs_id: service_address.id,
            service_id: service_address.service_id,
            name: service_info.unwrap().name,
            address: service_address.address,
        })
    }
}
