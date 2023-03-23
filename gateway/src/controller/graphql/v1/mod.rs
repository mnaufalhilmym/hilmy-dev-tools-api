use async_graphql::MergedObject;

use self::{
    account::{AccountMutation, AccountQuery},
    apprepo::{ApprepoMutation, ApprepoQuery},
    link::{LinkMutation, LinkQuery},
    service_address::{ServiceAddressMutation, ServiceAddressQuery},
    service_info::{ServiceInfoMutation, ServiceInfoQuery},
};

mod account;
mod apprepo;
mod link;
mod service_address;
mod service_info;

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
