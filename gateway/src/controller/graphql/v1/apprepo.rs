use std::str::FromStr;

use async_graphql::{Context, Object, Result};
use tonic::Request;
use tools_apprepo::proto::{self, apprepo::ApprepoServiceClient};
use tools_lib_db::pg::connection::DbPool;
use uuid::Uuid;

use crate::{
    contract::graphql::{apprepo::Apprepo, op_res::OpRes},
    dto::token::Token,
    env::AppMode,
    helper, service,
};

#[derive(Default)]
pub struct ApprepoQuery;

#[Object]
impl ApprepoQuery {
    async fn apprepos<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Apprepo>> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>(),
            ctx.data_unchecked::<DbPool>(),
        )?;

        let mut client =
            ApprepoServiceClient::new(service::grpc::client::get(db_conn, "apprepo").await?);

        let res = client
            .get_apprepos(Request::new(proto::apprepo::GetAppreposReq {}))
            .await?;

        Ok(res
            .get_ref()
            .apprepos
            .iter()
            .map(|apprepo| Apprepo {
                id: Uuid::from_str(&apprepo.id).unwrap(),
                name: apprepo.name.to_owned(),
                icon: apprepo.icon.to_owned(),
                link: apprepo.link.to_owned(),
                created_at: apprepo.created_at.to_owned(),
                updated_at: apprepo.updated_at.to_owned(),
            })
            .collect())
    }
}

#[derive(Default)]
pub struct ApprepoMutation;

#[Object]
impl ApprepoMutation {
    async fn create_apprepo<'a>(
        &self,
        ctx: &Context<'a>,
        name: String,
        icon: String,
        link: String,
    ) -> Result<Apprepo> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let token = ctx
            .data_opt::<Token>()
            .ok_or("Token doesn't exist")?
            .0
            .to_owned();

        if !helper::is_admin(db_conn, token).await? {
            return Err("Forbidden".into());
        }

        let mut client =
            ApprepoServiceClient::new(service::grpc::client::get(db_conn, "apprepo").await?);

        let res = client
            .create_apprepo(Request::new(proto::apprepo::CreateApprepoReq {
                name,
                icon,
                link,
            }))
            .await?;

        Ok(Apprepo {
            id: Uuid::from_str(&res.get_ref().id)?,
            name: res.get_ref().name.to_owned(),
            icon: res.get_ref().icon.to_owned(),
            link: res.get_ref().link.to_owned(),
            created_at: res.get_ref().created_at.to_owned(),
            updated_at: res.get_ref().updated_at.to_owned(),
        })
    }

    async fn update_apprepo<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        name: Option<String>,
        icon: Option<String>,
        link: Option<String>,
    ) -> Result<Apprepo> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let token = ctx
            .data_opt::<Token>()
            .ok_or("Token doesn't exist")?
            .0
            .to_owned();

        if !helper::is_admin(db_conn, token).await? {
            return Err("Forbidden".into());
        }

        let mut client =
            ApprepoServiceClient::new(service::grpc::client::get(db_conn, "apprepo").await?);

        let res = client
            .update_apprepo(Request::new(proto::apprepo::UpdateApprepoReq {
                id: id.to_string(),
                name,
                icon,
                link,
            }))
            .await?;

        Ok(Apprepo {
            id: Uuid::from_str(&res.get_ref().id)?,
            name: res.get_ref().name.to_owned(),
            icon: res.get_ref().icon.to_owned(),
            link: res.get_ref().link.to_owned(),
            created_at: res.get_ref().created_at.to_owned(),
            updated_at: res.get_ref().updated_at.to_owned(),
        })
    }

    async fn delete_apprepo<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<OpRes> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let token = ctx
            .data_opt::<Token>()
            .ok_or("Token doesn't exist")?
            .0
            .to_owned();

        if !helper::is_admin(db_conn, token).await? {
            return Err("Forbidden".into());
        }

        let mut client =
            ApprepoServiceClient::new(service::grpc::client::get(db_conn, "apprepo").await?);

        let res = client
            .delete_apprepo(Request::new(proto::apprepo::DeleteApprepoReq {
                id: id.to_string(),
            }))
            .await?;

        Ok(OpRes {
            is_success: res.get_ref().is_success,
        })
    }
}
