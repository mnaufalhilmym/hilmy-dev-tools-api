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

    pub fn kafka_addrs() -> String {
        env::var("KAFKA_ADDRS").unwrap()
    }

    pub fn kafka_group_id() -> String {
        env::var("KAFKA_GROUP_ID").unwrap()
    }

    pub fn kafka_input_topic() -> String {
        env::var("KAFKA_INPUT_TOPIC").unwrap()
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
