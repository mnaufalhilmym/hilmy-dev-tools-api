use async_graphql::{Context, Object, Result};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use tools_lib_db::pg::connection::DbPool;
use uuid::Uuid;

use crate::{
    contract::graphql::{op_res::OpRes, service_info::ServiceInfo},
    dto::token::Token,
    env::{AppMode, GrpcConnectTimeout},
    helper, model, schema,
};

#[derive(Default)]
pub struct ServiceInfoQuery;

#[Object]
impl ServiceInfoQuery {
    async fn services_info<'a>(
        &self,
        ctx: &Context<'a>,
        name: Option<String>,
    ) -> Result<Vec<ServiceInfo>> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();
        let token = ctx
            .data_opt::<Token>()
            .ok_or("Token doesn't exist")?
            .0
            .to_owned();

        if !helper::is_admin(db_conn, token, grpc_connect_timeout).await? {
            return Err("Forbidden".into());
        }

        let mut query = schema::service_info::table.into_boxed();
        if let Some(name) = name {
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

    async fn service_info<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<ServiceInfo> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();
        let token = ctx
            .data_opt::<Token>()
            .ok_or("Token doesn't exist")?
            .0
            .to_owned();

        if !helper::is_admin(db_conn, token, grpc_connect_timeout).await? {
            return Err("Forbidden".into());
        }

        let service_info = schema::service_info::table
            .find(&id)
            .first::<model::ServiceInfo>(db_conn)?;

        Ok(ServiceInfo {
            id: service_info.id,
            name: service_info.name,
            created_at: service_info.created_at,
            updated_at: service_info.created_at,
        })
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
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();
        let token = ctx
            .data_opt::<Token>()
            .ok_or("Token doesn't exist")?
            .0
            .to_owned();

        if !helper::is_admin(db_conn, token, grpc_connect_timeout).await? {
            return Err("Forbidden".into());
        }

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
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();
        let token = ctx
            .data_opt::<Token>()
            .ok_or("Token doesn't exist")?
            .0
            .to_owned();

        if !helper::is_admin(db_conn, token, grpc_connect_timeout).await? {
            return Err("Forbidden".into());
        }

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

    async fn delete_service_info<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<OpRes> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();
        let token = ctx
            .data_opt::<Token>()
            .ok_or("Token doesn't exist")?
            .0
            .to_owned();

        if !helper::is_admin(db_conn, token, grpc_connect_timeout).await? {
            return Err("Forbidden".into());
        }

        diesel::delete(schema::service_info::table.find(id)).execute(db_conn)?;

        Ok(OpRes { is_success: true })
    }
}
