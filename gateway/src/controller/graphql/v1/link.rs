use std::str::FromStr;

use async_graphql::{Context, Object, Result};
use tonic::Request;
use tools_lib_db::pg::connection::DbPool;
use tools_link::proto::link::LinkServiceClient;
use uuid::Uuid;

use crate::{
    contract::graphql::{
        link::{GetLinkByShortUrlRes, Link, VisitLinkRes},
        op_res::OpRes,
    },
    dto::{service_name::ServiceName, token::Token},
    env::{AppMode, GrpcConnectTimeout},
    helper::get_account_id,
    service,
};

#[derive(Default)]
pub struct LinkQuery;

#[Object]
impl LinkQuery {
    async fn links<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Link>> {
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

        let account_id = get_account_id(db_conn, token, grpc_connect_timeout).await?;

        let mut client = LinkServiceClient::new(
            service::grpc::client::get(db_conn, &ServiceName::link(), grpc_connect_timeout).await?,
        );

        let res = client
            .get_links(Request::new(tools_link::proto::link::GetLinksReq {
                created_by_id: account_id,
            }))
            .await?;

        Ok(res
            .get_ref()
            .links
            .iter()
            .map(|link| Link {
                id: Uuid::from_str(&link.id).unwrap(),
                title: link.title.to_owned(),
                short_url: link.short_url.to_owned(),
                long_url: link.long_url.to_owned(),
                visits: link.visits,
                created_at: link.created_at.to_owned(),
                updated_at: link.updated_at.to_owned(),
            })
            .collect())
    }

    async fn link<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Link> {
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

        let account_id = get_account_id(db_conn, token, grpc_connect_timeout).await?;

        let mut client = LinkServiceClient::new(
            service::grpc::client::get(db_conn, &ServiceName::link(), grpc_connect_timeout).await?,
        );

        let res = client
            .get_link(Request::new(tools_link::proto::link::GetLinkReq {
                id: id.to_string(),
                created_by_id: account_id,
            }))
            .await?;

        Ok(Link {
            id: Uuid::from_str(&res.get_ref().id)?,
            title: res.get_ref().title.to_owned(),
            short_url: res.get_ref().short_url.to_owned(),
            long_url: res.get_ref().long_url.to_owned(),
            visits: res.get_ref().visits,
            created_at: res.get_ref().created_at.to_owned(),
            updated_at: res.get_ref().updated_at.to_owned(),
        })
    }

    async fn link_by_short_url<'a>(
        &self,
        ctx: &Context<'a>,
        short_url: String,
    ) -> Result<GetLinkByShortUrlRes> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();

        let mut client = LinkServiceClient::new(
            service::grpc::client::get(db_conn, &ServiceName::link(), grpc_connect_timeout).await?,
        );

        let res = client
            .get_link_by_short_url(Request::new(
                tools_link::proto::link::GetLinkByShortUrlReq { short_url },
            ))
            .await?;

        Ok(GetLinkByShortUrlRes {
            short_url: res.get_ref().short_url.to_owned(),
            long_url: res.get_ref().long_url.to_owned(),
        })
    }

    async fn visit_link<'a>(&self, ctx: &Context<'a>, short_url: String) -> Result<VisitLinkRes> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();

        let mut client = LinkServiceClient::new(
            service::grpc::client::get(db_conn, &ServiceName::link(), grpc_connect_timeout).await?,
        );

        let res = client
            .visit_link(Request::new(tools_link::proto::link::VisitLinkReq {
                short_url,
            }))
            .await?;

        Ok(VisitLinkRes {
            short_url: res.get_ref().short_url.to_owned(),
            long_url: res.get_ref().long_url.to_owned(),
        })
    }
}

#[derive(Default)]
pub struct LinkMutation;

#[Object]
impl LinkMutation {
    async fn create_link<'a>(
        &self,
        ctx: &Context<'a>,
        title: String,
        short_url: String,
        long_url: String,
    ) -> Result<Link> {
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

        let account_id = get_account_id(db_conn, token, grpc_connect_timeout).await?;

        let mut client = LinkServiceClient::new(
            service::grpc::client::get(db_conn, &ServiceName::link(), grpc_connect_timeout).await?,
        );

        let res = client
            .create_link(Request::new(tools_link::proto::link::CreateLinkReq {
                title,
                short_url,
                long_url,
                created_by_id: account_id,
            }))
            .await?;

        Ok(Link {
            id: Uuid::from_str(&res.get_ref().id)?,
            title: res.get_ref().title.to_owned(),
            short_url: res.get_ref().short_url.to_owned(),
            long_url: res.get_ref().long_url.to_owned(),
            visits: res.get_ref().visits,
            created_at: res.get_ref().created_at.to_owned(),
            updated_at: res.get_ref().updated_at.to_owned(),
        })
    }

    async fn update_link<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        title: Option<String>,
        short_url: Option<String>,
        long_url: Option<String>,
    ) -> Result<Link> {
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

        let account_id = get_account_id(db_conn, token, grpc_connect_timeout).await?;

        let mut client = LinkServiceClient::new(
            service::grpc::client::get(db_conn, &ServiceName::link(), grpc_connect_timeout).await?,
        );

        let res = client
            .update_link(Request::new(tools_link::proto::link::UpdateLinkReq {
                id: id.to_string(),
                title,
                short_url,
                long_url,
                created_by_id: account_id,
            }))
            .await?;

        Ok(Link {
            id: Uuid::from_str(&res.get_ref().id)?,
            title: res.get_ref().title.to_owned(),
            short_url: res.get_ref().short_url.to_owned(),
            long_url: res.get_ref().long_url.to_owned(),
            visits: res.get_ref().visits,
            created_at: res.get_ref().created_at.to_owned(),
            updated_at: res.get_ref().updated_at.to_owned(),
        })
    }

    async fn delete_link<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<OpRes> {
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

        let account_id = get_account_id(db_conn, token, grpc_connect_timeout).await?;

        let mut client = LinkServiceClient::new(
            service::grpc::client::get(db_conn, &ServiceName::link(), grpc_connect_timeout).await?,
        );

        let res = client
            .delete_link(Request::new(tools_link::proto::link::DeleteLinkReq {
                id: id.to_string(),
                created_by_id: account_id,
            }))
            .await?;

        Ok(OpRes {
            is_success: res.get_ref().is_success,
        })
    }
}
