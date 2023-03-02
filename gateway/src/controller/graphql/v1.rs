use async_graphql::MergedObject;

use self::{
    account::AccountMutation,
    service::{ServiceAddressQuery, ServiceInfoQuery, ServiceMutation},
};

mod account;
mod service;

#[derive(MergedObject, Default)]
pub struct QueryRootV1(ServiceInfoQuery, ServiceAddressQuery);

#[derive(MergedObject, Default)]
pub struct MutationRootV1(ServiceMutation, AccountMutation);
