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

    pub fn database_url() -> String {
        env::var("DATABASE_URL").unwrap()
    }

    pub fn redis_url() -> String {
        env::var("REDIS_URL").unwrap()
    }

    pub fn hash_secret() -> String {
        env::var("HASH_SECRET").unwrap()
    }

    pub fn kafka_addrs() -> String {
        env::var("KAFKA_ADDRS").unwrap()
    }
}
