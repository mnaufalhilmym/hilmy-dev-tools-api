use core::fmt;
use std::env;

pub struct Env;

impl Env {
    pub fn app_name() -> AppName {
        AppName(env::var("APP_NAME").unwrap())
    }

    pub fn app_mode() -> AppMode {
        AppMode(env::var("APP_MODE").unwrap())
    }

    pub fn service_name() -> ServiceName {
        ServiceName(env::var("SERVICE_NAME").unwrap())
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

pub type GrpcConnectTimeout = u64;

#[derive(Clone)]
pub struct AppName(String);

impl fmt::Display for AppName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.0, f)
    }
}

impl AppName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone)]
pub struct ServiceName(String);

impl fmt::Display for ServiceName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.0, f)
    }
}

impl ServiceName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(PartialEq, Clone)]
pub struct AppMode(String);

impl fmt::Display for AppMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.0, f)
    }
}

impl AppMode {
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
