use std::error::Error;

use tonic::Request;
use tools_account::proto::account::AccountServiceClient;
use tools_lib_db::pg::connection::DbPooled;

use crate::{dto::service_name::ServiceName, service};

pub async fn get_account_id(
    db_conn: &mut DbPooled,
    token: String,
    grpc_connect_timeout: &u64,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut client = AccountServiceClient::new(
        service::grpc::client::get(db_conn, &ServiceName::account(), grpc_connect_timeout).await?,
    );

    Ok(client
        .validate_token(Request::new(
            tools_account::proto::account::ValidateTokenReq { token },
        ))
        .await?
        .get_ref()
        .id
        .to_owned())
}
