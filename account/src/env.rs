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

    pub fn argon2_hash_secret() -> String {
        env::var("ARGON2_HASH_SECRET").unwrap()
    }

    pub fn jwt_secret() -> String {
        env::var("JWT_SECRET").unwrap()
    }

    pub fn use_msg_broker() -> UseMsgBroker {
        UseMsgBroker(env::var("USE_MSG_BROKER").unwrap())
    }

    pub fn kafka_addrs() -> String {
        env::var("KAFKA_ADDRS").unwrap()
    }

    pub fn kafka_msg_timeout() -> String {
        env::var("KAFKA_MSG_TIMEOUT").unwrap()
    }

    pub fn rabbitmq_addrs() -> String {
        env::var("RABBITMQ_ADDRS").unwrap()
    }
}

pub struct UseMsgBroker(String);

impl UseMsgBroker {
    pub fn is_kafka(&self) -> bool {
        self.0 == "KAFKA"
    }

    pub fn is_rabbitmq(&self) -> bool {
        self.0 == "RABBITMQ"
    }
}
