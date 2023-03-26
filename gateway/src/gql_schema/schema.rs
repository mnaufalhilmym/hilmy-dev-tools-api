use async_graphql::{EmptySubscription, MergedObject, Schema};
use tools_lib_db::pg::connection::DbPool;

use crate::{
    controller::graphql::v1::{
        account::{AccountMutation, AccountQuery},
        apprepo::{ApprepoMutation, ApprepoQuery},
        link::{LinkMutation, LinkQuery},
        service_address::{ServiceAddressMutation, ServiceAddressQuery},
        service_info::{ServiceInfoMutation, ServiceInfoQuery},
    },
    env::{AppMode, GrpcConnectTimeout},
};

#[derive(MergedObject, Default)]
pub struct QueryRootV1(
    ServiceInfoQuery,
    ServiceAddressQuery,
    AccountQuery,
    LinkQuery,
    ApprepoQuery,
);

#[derive(MergedObject, Default)]
pub struct MutationRootV1(
    ServiceInfoMutation,
    ServiceAddressMutation,
    AccountMutation,
    LinkMutation,
    ApprepoMutation,
);

pub type GqlSchema = Schema<QueryRootV1, MutationRootV1, EmptySubscription>;

pub struct GqlData {
    pub app_mode: AppMode,
    pub db_pool: DbPool,
    pub grpc_connect_timeout: GrpcConnectTimeout,
}

pub fn build_gql_schema(data: GqlData) -> GqlSchema {
    let mut schema = Schema::build(
        QueryRootV1::default(),
        MutationRootV1::default(),
        EmptySubscription,
    );

    if data.app_mode.is_release() {
        schema = schema.disable_introspection().disable_suggestions()
    }

    schema
        .data(data.app_mode)
        .data(data.db_pool)
        .data(data.grpc_connect_timeout)
        .finish()
}
