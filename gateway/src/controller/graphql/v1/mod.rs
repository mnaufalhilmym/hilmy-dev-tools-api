use async_graphql::MergedObject;

use self::{
    account::{AccountMutation, AccountQuery},
    link::{LinkMutation, LinkQuery},
    service_address::{ServiceAddressMutation, ServiceAddressQuery},
    service_info::{ServiceInfoMutation, ServiceInfoQuery},
};

mod account;
mod link;
mod service_address;
mod service_info;

#[derive(MergedObject, Default)]
pub struct QueryRootV1(
    ServiceInfoQuery,
    ServiceAddressQuery,
    AccountQuery,
    LinkQuery,
);

#[derive(MergedObject, Default)]
pub struct MutationRootV1(
    ServiceInfoMutation,
    ServiceAddressMutation,
    AccountMutation,
    LinkMutation,
);
