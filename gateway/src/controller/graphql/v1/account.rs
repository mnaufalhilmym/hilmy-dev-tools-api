use async_graphql::{Context, Object, Result};
use tonic::Request;
use tools_account::proto::{self, account::AccountServiceClient};
use tools_lib_db::pg::connection::DbPool;

use crate::{env::AppMode, service};

struct OpResult {
    is_success: bool,
}

#[Object]
impl OpResult {
    async fn is_success(&self) -> &bool {
        &self.is_success
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
}
