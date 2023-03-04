use async_graphql::MergedObject;

use self::{
    account::AccountMutation,
    service_address::{ServiceAddressMutation, ServiceAddressQuery},
    service_info::{ServiceInfoMutation, ServiceInfoQuery},
};

mod account;
mod service_address;
mod service_info;

#[derive(MergedObject, Default)]
pub struct QueryRootV1(ServiceInfoQuery, ServiceInfoQuery, ServiceAddressQuery);

#[derive(MergedObject, Default)]
pub struct MutationRootV1(ServiceInfoMutation, ServiceAddressMutation, AccountMutation);
