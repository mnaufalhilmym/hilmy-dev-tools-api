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
    env::AppMode,
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
}

pub fn build_gql_schema(data: GqlData) -> GqlSchema {
    Schema::build(
        QueryRootV1::default(),
        MutationRootV1::default(),
        EmptySubscription,
    )
    .data(data.app_mode)
    .data(data.db_pool)
    .finish()
}
