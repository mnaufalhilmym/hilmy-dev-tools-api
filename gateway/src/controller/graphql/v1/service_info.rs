use async_graphql::{Context, Object, Result};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use tools_db::pg::connection::DbPool;
use uuid::Uuid;

use crate::{controller::graphql::generic_schema::OpRes, env::AppMode, model, schema};

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
        let db_conn = &mut tools_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>(),
            ctx.data_unchecked::<DbPool>(),
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

#[derive(Default)]
pub struct ServiceInfoMutation;

#[Object]
impl ServiceInfoMutation {
    async fn create_service_info<'a>(
        &self,
        ctx: &Context<'a>,
        name: String,
    ) -> Result<ServiceInfo> {
        let db_conn = &mut tools_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>(),
            ctx.data_unchecked::<DbPool>(),
        )?;

        let service_info = diesel::insert_into(schema::service_info::table)
            .values(schema::service_info::name.eq(&name))
            .get_result::<model::ServiceInfo>(db_conn)?;

        Ok(ServiceInfo {
            id: service_info.id,
            name: service_info.name,
            created_at: service_info.created_at,
            updated_at: service_info.updated_at,
        })
    }

    async fn update_service_info<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        name: String,
    ) -> Result<ServiceInfo> {
        let db_conn = &mut tools_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>(),
            ctx.data_unchecked::<DbPool>(),
        )?;

        let service_info = diesel::update(schema::service_info::table.find(id))
            .set((
                schema::service_info::name.eq(&name),
                schema::service_info::updated_at.eq(&diesel::dsl::now),
            ))
            .get_result::<model::ServiceInfo>(db_conn)?;

        Ok(ServiceInfo {
            id: service_info.id,
            name: service_info.name,
            created_at: service_info.created_at,
            updated_at: service_info.updated_at,
        })
    }

    async fn delete_service_address<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<OpRes> {
        let db_conn = &mut tools_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>(),
            ctx.data_unchecked::<DbPool>(),
        )?;

        diesel::delete(schema::service_info::table.find(id)).execute(db_conn)?;

        Ok(OpRes { is_success: true })
    }
}
