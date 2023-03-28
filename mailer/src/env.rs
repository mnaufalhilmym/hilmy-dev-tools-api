use std::env;

pub struct Env;

impl Env {
    pub fn app_mode() -> String {
        env::var("APP_MODE").unwrap()
    }

    pub fn app_name() -> String {
        env::var("APP_NAME").unwrap()
    }

    pub fn service_name() -> String {
        env::var("SERVICE_NAME").unwrap()
    }

    pub fn use_msg_broker() -> UseMsgBroker {
        UseMsgBroker(env::var("USE_MSG_BROKER").unwrap())
    }

    pub fn msg_broker_consume() -> String {
        env::var("MSG_BROKER_CONSUME").unwrap()
    }

    pub fn kafka_addrs() -> String {
        env::var("KAFKA_ADDRS").unwrap()
    }

    pub fn kafka_group_id() -> String {
        env::var("KAFKA_GROUP_ID").unwrap()
    }

    pub fn rabbitmq_addrs() -> String {
        env::var("RABBITMQ_ADDRS").unwrap()
    }

    pub fn smtp_server() -> String {
        env::var("SMTP_SERVER").unwrap()
    }

    pub fn smtp_username() -> String {
        env::var("SMTP_USERNAME").unwrap()
    }

    pub fn smtp_password() -> String {
        env::var("SMTP_PASSWORD").unwrap()
    }

    pub fn sender_name() -> String {
        env::var("SENDER_NAME").unwrap()
    }

    pub fn sender_email() -> String {
        env::var("SENDER_EMAIL").unwrap()
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
