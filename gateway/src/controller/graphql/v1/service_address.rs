use async_graphql::{Context, Object, Result};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use tools_lib_db::pg::connection::DbPool;
use uuid::Uuid;

use crate::{controller::graphql::generic_schema::OpRes, env::AppMode, model, schema, service};

struct ServiceAddress {
    id: Uuid,
    service_id: Uuid,
    address: String,
    status: model::ServiceAddressStatus,
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

    async fn status(&self) -> &model::ServiceAddressStatus {
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
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>(),
            ctx.data_unchecked::<DbPool>(),
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

#[derive(Default)]
pub struct ServiceAddressMutation;

#[Object]
impl ServiceAddressMutation {
    async fn create_service_address<'a>(
        &self,
        ctx: &Context<'a>,
        service_id: Uuid,
        address: String,
    ) -> Result<ServiceAddress> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>(),
            ctx.data_unchecked::<DbPool>(),
        )?;

        let service_address = diesel::insert_into(schema::service_address::table)
            .values((
                schema::service_address::service_id.eq(&service_id),
                schema::service_address::address.eq(&address),
                schema::service_address::status.eq(&model::ServiceAddressStatus::Inaccessible),
                schema::service_address::last_used_at.eq(&diesel::dsl::now),
            ))
            .get_result::<model::ServiceAddress>(db_conn)?;

        Ok(ServiceAddress {
            id: service_address.id,
            service_id: service_address.service_id,
            address: service_address.address,
            status: service_address.status,
            last_used_at: service_address.last_used_at,
            created_at: service_address.created_at,
            updated_at: service_address.updated_at,
        })
    }

    async fn update_service_address<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        service_id: Option<Uuid>,
        address: Option<String>,
        status: Option<model::ServiceAddressStatus>,
    ) -> Result<ServiceAddress> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>(),
            ctx.data_unchecked::<DbPool>(),
        )?;

        let mut change_set = model::ServiceAddressChangeSet {
            service_id,
            address: address.to_owned(),
            status,
            ..Default::default()
        };

        if let Some(status) = status {
            if status == model::ServiceAddressStatus::Accessible {
                let address = match address {
                    Some(address) => address,
                    None => {
                        schema::service_address::table
                            .find(&id)
                            .first::<model::ServiceAddress>(db_conn)?
                            .address
                    }
                };
                let client = service::grpc::client::connect(&address).await;
                if let Err(e) = client {
                    return Err(format!("Can't reach {}. Error: {}", address, e).into());
                }
                change_set.last_used_at = Some(chrono::Utc::now().naive_utc());
            }
        }

        let service_address = diesel::update(schema::service_address::table.find(id))
            .set((
                change_set,
                schema::service_address::updated_at.eq(&diesel::dsl::now),
            ))
            .get_result::<model::ServiceAddress>(db_conn)?;

        Ok(ServiceAddress {
            id: service_address.id,
            service_id: service_address.service_id,
            address: service_address.address,
            status: service_address.status,
            last_used_at: service_address.last_used_at,
            created_at: service_address.created_at,
            updated_at: service_address.updated_at,
        })
    }

    async fn delete_service_address<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<OpRes> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>(),
            ctx.data_unchecked::<DbPool>(),
        )?;

        diesel::delete(schema::service_address::table.find(id)).execute(db_conn)?;

        Ok(OpRes { is_success: true })
    }
}
