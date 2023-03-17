use async_graphql::{Context, Object, Result};
use tonic::Request;
use tools_account::proto::{self, account::AccountServiceClient};
use tools_lib_db::pg::connection::DbPool;

use crate::{
    contract::graphql::{op_result::OpResult, sign_in_result::SignInResult},
    dto::token::Token,
    env::AppMode,
    service,
};

#[derive(Default)]
pub struct AccountMutation;

#[Object]
impl AccountMutation {
    async fn sign_up<'a>(
        &self,
        ctx: &Context<'a>,
        email: String,
        password: String,
    ) -> Result<OpResult> {
        let app_mode = ctx.data_unchecked::<AppMode>();
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            &app_mode,
            &ctx.data_unchecked::<DbPool>(),
        )?;

        let mut client =
            AccountServiceClient::new(service::grpc::client::get(db_conn, "account").await?);

        let res = client
            .sign_up(Request::new(proto::account::SignUpReq { email, password }))
            .await?;

        Ok(OpResult {
            is_success: res.get_ref().is_success,
        })
    }

    async fn verify_sign_up<'a>(
        &self,
        ctx: &Context<'a>,
        email: String,
        verify_code: String,
    ) -> Result<OpResult> {
        let app_mode = ctx.data_unchecked::<AppMode>();
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            &app_mode,
            &ctx.data_unchecked::<DbPool>(),
        )?;

        let mut client =
            AccountServiceClient::new(service::grpc::client::get(db_conn, "account").await?);

        let res = client
            .verify_sign_up(Request::new(proto::account::VerifySignUpReq {
                email,
                verify_code,
            }))
            .await?;

        Ok(OpResult {
            is_success: res.get_ref().is_success,
        })
    }

    async fn sign_in<'a>(
        &self,
        ctx: &Context<'a>,
        email: String,
        password: String,
    ) -> Result<SignInResult> {
        let app_mode = ctx.data_unchecked::<AppMode>();
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            &app_mode,
            &ctx.data_unchecked::<DbPool>(),
        )?;

        let mut client =
            AccountServiceClient::new(service::grpc::client::get(db_conn, "account").await?);

        let res = client
            .sign_in(Request::new(proto::account::SignInReq { email, password }))
            .await?;

        Ok(SignInResult {
            token: res.get_ref().token.to_owned(),
        })
    }

    async fn change_email<'a>(&self, ctx: &Context<'a>, new_email: String) -> Result<OpResult> {
        let app_mode = ctx.data_unchecked::<AppMode>();
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            &app_mode,
            &ctx.data_unchecked::<DbPool>(),
        )?;
        let token = ctx
            .data_opt::<Token>()
            .ok_or("Token doesn't exist")?
            .0
            .to_owned();

        let mut client =
            AccountServiceClient::new(service::grpc::client::get(db_conn, "account").await?);

        let res = client
            .change_email(Request::new(proto::account::ChangeEmailReq {
                token,
                new_email,
            }))
            .await?;

        Ok(OpResult {
            is_success: res.get_ref().is_success,
        })
    }

    async fn verify_change_email<'a>(
        &self,
        ctx: &Context<'a>,
        new_email: String,
        verify_code: String,
    ) -> Result<OpResult> {
        let app_mode = ctx.data_unchecked::<AppMode>();
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            &app_mode,
            &ctx.data_unchecked::<DbPool>(),
        )?;

        let mut client =
            AccountServiceClient::new(service::grpc::client::get(db_conn, "account").await?);

        let res = client
            .verify_change_email(Request::new(proto::account::VerifyChangeEmailReq {
                new_email,
                verify_code,
            }))
            .await?;

        Ok(OpResult {
            is_success: res.get_ref().is_success,
        })
    }

    async fn change_password<'a>(
        &self,
        ctx: &Context<'a>,
        old_password: String,
        new_password: String,
    ) -> Result<OpResult> {
        let app_mode = ctx.data_unchecked::<AppMode>();
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            &app_mode,
            &ctx.data_unchecked::<DbPool>(),
        )?;
        let token = ctx
            .data_opt::<Token>()
            .ok_or("Token doesn't exist")?
            .0
            .to_owned();

        let mut client =
            AccountServiceClient::new(service::grpc::client::get(db_conn, "account").await?);

        let res = client
            .change_password(Request::new(proto::account::ChangePasswordReq {
                token,
                old_password,
                new_password,
            }))
            .await?;

        Ok(OpResult {
            is_success: res.get_ref().is_success,
        })
    }

    async fn request_reset_password<'a>(
        &self,
        ctx: &Context<'a>,
        email: String,
    ) -> Result<OpResult> {
        let app_mode = ctx.data_unchecked::<AppMode>();
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            &app_mode,
            &ctx.data_unchecked::<DbPool>(),
        )?;

        let mut client =
            AccountServiceClient::new(service::grpc::client::get(db_conn, "account").await?);

        let res = client
            .request_reset_password(Request::new(proto::account::RequestResetPasswordReq {
                email,
            }))
            .await?;

        Ok(OpResult {
            is_success: res.get_ref().is_success,
        })
    }

    async fn verify_request_reset_password<'a>(
        &self,
        ctx: &Context<'a>,
        email: String,
        verify_code: String,
    ) -> Result<OpResult> {
        let app_mode = ctx.data_unchecked::<AppMode>();
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            &app_mode,
            &ctx.data_unchecked::<DbPool>(),
        )?;

        let mut client =
            AccountServiceClient::new(service::grpc::client::get(db_conn, "account").await?);

        let res = client
            .verify_request_reset_password(Request::new(
                proto::account::VerifyRequestResetPasswordReq { email, verify_code },
            ))
            .await?;

        Ok(OpResult {
            is_success: res.get_ref().is_success,
        })
    }

    async fn reset_password<'a>(
        &self,
        ctx: &Context<'a>,
        email: String,
        verify_code: String,
        new_password: String,
    ) -> Result<OpResult> {
        let app_mode = ctx.data_unchecked::<AppMode>();
        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
            &app_mode,
            &ctx.data_unchecked::<DbPool>(),
        )?;

        let mut client =
            AccountServiceClient::new(service::grpc::client::get(db_conn, "account").await?);

        let res = client
            .reset_password(Request::new(proto::account::ResetPasswordReq {
                email,
                verify_code,
                new_password,
            }))
            .await?;

        Ok(OpResult {
            is_success: res.get_ref().is_success,
        })
    }
}
