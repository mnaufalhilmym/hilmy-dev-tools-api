use std::env;

pub struct Env;

impl Env {
    pub fn app_name() -> String {
        env::var("APP_NAME").unwrap()
    }

    pub fn app_mode() -> String {
        env::var("APP_MODE").unwrap()
    }

    pub fn service_name() -> String {
        env::var("SERVICE_NAME").unwrap()
    }

    pub fn service_addrs() -> String {
        env::var("SERVICE_ADDRS").unwrap()
    }

    pub fn service_gql_addrs() -> String {
        env::var("SERVICE_GQL_ADDRS").unwrap()
    }

    pub fn site_link_url() -> String {
        env::var("SITE_LINK_URL").unwrap()
    }
}
