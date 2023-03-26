use std::str::FromStr;

use async_graphql::{Context, Object, Result};
use tonic::Request;
use tools_account::proto::{self, account::AccountServiceClient};
use tools_lib_db::pg::connection::DbPool;
use uuid::Uuid;

use crate::{
    contract::graphql::{
        account::{Account, SignInResult},
        op_res::OpRes,
    },
    dto::token::Token,
    env::{AppMode, GrpcConnectTimeout},
    service,
};

#[derive(Default)]
pub struct AccountQuery;

#[Object]
impl AccountQuery {
    async fn account<'a>(&self, ctx: &Context<'a>) -> Result<Account> {
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

        let mut client = AccountServiceClient::new(
            service::grpc::client::get(db_conn, "account", grpc_connect_timeout).await?,
        );

        let res = client
            .get_account(Request::new(proto::account::GetAccountReq { token }))
            .await?;

        Ok(Account {
            id: Uuid::from_str(&res.get_ref().id)?,
            email: res.get_ref().email.to_owned(),
            created_at: res.get_ref().created_at.to_owned(),
            updated_at: res.get_ref().updated_at.to_owned(),
        })
    }
}

#[derive(Default)]
pub struct AccountMutation;

#[Object]
impl AccountMutation {
    async fn sign_up<'a>(
        &self,
        ctx: &Context<'a>,
        email: String,
        password: String,
    ) -> Result<OpRes> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();

        let mut client = AccountServiceClient::new(
            service::grpc::client::get(db_conn, "account", grpc_connect_timeout).await?,
        );

        let res = client
            .sign_up(Request::new(proto::account::SignUpReq { email, password }))
            .await?;

        Ok(OpRes {
            is_success: res.get_ref().is_success,
        })
    }

    async fn verify_sign_up<'a>(
        &self,
        ctx: &Context<'a>,
        email: String,
        verify_code: String,
    ) -> Result<OpRes> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();

        let mut client = AccountServiceClient::new(
            service::grpc::client::get(db_conn, "account", grpc_connect_timeout).await?,
        );

        let res = client
            .verify_sign_up(Request::new(proto::account::VerifySignUpReq {
                email,
                verify_code,
            }))
            .await?;

        Ok(OpRes {
            is_success: res.get_ref().is_success,
        })
    }

    async fn sign_in<'a>(
        &self,
        ctx: &Context<'a>,
        email: String,
        password: String,
    ) -> Result<SignInResult> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();

        let mut client = AccountServiceClient::new(
            service::grpc::client::get(db_conn, "account", grpc_connect_timeout).await?,
        );

        let res = client
            .sign_in(Request::new(proto::account::SignInReq { email, password }))
            .await?;

        Ok(SignInResult {
            token: res.get_ref().token.to_owned(),
        })
    }

    async fn change_email<'a>(&self, ctx: &Context<'a>, new_email: String) -> Result<OpRes> {
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

        let mut client = AccountServiceClient::new(
            service::grpc::client::get(db_conn, "account", grpc_connect_timeout).await?,
        );

        let res = client
            .change_email(Request::new(proto::account::ChangeEmailReq {
                token,
                new_email,
            }))
            .await?;

        Ok(OpRes {
            is_success: res.get_ref().is_success,
        })
    }

    async fn verify_change_email<'a>(
        &self,
        ctx: &Context<'a>,
        new_email: String,
        verify_code: String,
    ) -> Result<OpRes> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();

        let mut client = AccountServiceClient::new(
            service::grpc::client::get(db_conn, "account", grpc_connect_timeout).await?,
        );

        let res = client
            .verify_change_email(Request::new(proto::account::VerifyChangeEmailReq {
                new_email,
                verify_code,
            }))
            .await?;

        Ok(OpRes {
            is_success: res.get_ref().is_success,
        })
    }

    async fn change_password<'a>(
        &self,
        ctx: &Context<'a>,
        old_password: String,
        new_password: String,
    ) -> Result<OpRes> {
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

        let mut client = AccountServiceClient::new(
            service::grpc::client::get(db_conn, "account", grpc_connect_timeout).await?,
        );

        let res = client
            .change_password(Request::new(proto::account::ChangePasswordReq {
                token,
                old_password,
                new_password,
            }))
            .await?;

        Ok(OpRes {
            is_success: res.get_ref().is_success,
        })
    }

    async fn request_reset_password<'a>(&self, ctx: &Context<'a>, email: String) -> Result<OpRes> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();

        let mut client = AccountServiceClient::new(
            service::grpc::client::get(db_conn, "account", grpc_connect_timeout).await?,
        );

        let res = client
            .request_reset_password(Request::new(proto::account::RequestResetPasswordReq {
                email,
            }))
            .await?;

        Ok(OpRes {
            is_success: res.get_ref().is_success,
        })
    }

    async fn verify_request_reset_password<'a>(
        &self,
        ctx: &Context<'a>,
        email: String,
        verify_code: String,
    ) -> Result<OpRes> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();

        let mut client = AccountServiceClient::new(
            service::grpc::client::get(db_conn, "account", grpc_connect_timeout).await?,
        );

        let res = client
            .verify_request_reset_password(Request::new(
                proto::account::VerifyRequestResetPasswordReq { email, verify_code },
            ))
            .await?;

        Ok(OpRes {
            is_success: res.get_ref().is_success,
        })
    }

    async fn reset_password<'a>(
        &self,
        ctx: &Context<'a>,
        email: String,
        verify_code: String,
        new_password: String,
    ) -> Result<OpRes> {
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            ctx.data_unchecked::<AppMode>().as_str(),
            ctx.data_unchecked::<DbPool>(),
        )?;
        let grpc_connect_timeout = ctx.data_unchecked::<GrpcConnectTimeout>();

        let mut client = AccountServiceClient::new(
            service::grpc::client::get(db_conn, "account", grpc_connect_timeout).await?,
        );

        let res = client
            .reset_password(Request::new(proto::account::ResetPasswordReq {
                email,
                verify_code,
                new_password,
            }))
            .await?;

        Ok(OpRes {
            is_success: res.get_ref().is_success,
        })
    }

    async fn delete_account<'a>(&self, ctx: &Context<'a>) -> Result<OpRes> {
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

        let mut client = AccountServiceClient::new(
            service::grpc::client::get(db_conn, "account", grpc_connect_timeout).await?,
        );

        let res = client
            .delete_account(Request::new(proto::account::DeleteAccountReq { token }))
            .await?;

        Ok(OpRes {
            is_success: res.get_ref().is_success,
        })
    }
}
