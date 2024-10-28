use std::env;

use anyhow::{Error, Result};

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

    pub fn use_msg_broker() -> String {
        env::var("USE_MSG_BROKER").unwrap()
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

    pub fn rabbitmq_consumer_tag() -> String {
        env::var("RABBITMQ_CONSUMER_TAG").unwrap()
    }
}

pub enum UseMsgBroker {
    Kafka,
    Rabbitmq,
}

impl UseMsgBroker {
    pub fn from_string(string: String) -> Result<Self> {
        if string == "KAFKA" {
            Ok(Self::Kafka)
        } else if string == "RABBITMQ" {
            Ok(Self::Rabbitmq)
        } else {
            Err(Error::msg("Wrong value"))
        }
    }
}
