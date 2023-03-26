use core::fmt;
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

    pub fn grpc_connect_timeout() -> String {
        env::var("GRPC_CONNECT_TIMEOUT").unwrap()
    }
}

pub type AppName = String;
pub type ServiceName = String;
pub type GrpcConnectTimeout = u64;

#[derive(PartialEq, Clone)]
pub struct AppMode(pub String);

impl fmt::Display for AppMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.0, f)
    }
}

impl AppMode {
    pub fn from(v: String) -> Self {
        AppMode(v)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_release(&self) -> bool {
        self.0 == "RELEASE"
    }

    pub fn is_debug(&self) -> bool {
        self.0 == "DEBUG"
    }
}
