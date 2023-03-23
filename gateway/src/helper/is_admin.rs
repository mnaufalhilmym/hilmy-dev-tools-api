use std::error::Error;

use tonic::Request;
use tools_account::proto::account::AccountServiceClient;
use tools_lib_db::pg::connection::DbPooled;

use crate::service;

pub async fn is_admin(
    db_conn: &mut DbPooled,
    token: String,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let mut client =
        AccountServiceClient::new(service::grpc::client::get(db_conn, "account").await?);
    let account_role = client
        .validate_token(Request::new(
            tools_account::proto::account::ValidateTokenReq { token },
        ))
        .await?
        .get_ref()
        .role
        .to_owned();

    // 1 for admin role
    if account_role == 1 {
        return Ok(true);
    }

    Ok(false)
}
