use std::{future::Future, pin::Pin};

use anyhow::Result;

pub mod kafka_consumer;
pub mod rabbitmq_consumer;

pub enum Consumer {
    Kafka(Pin<Box<dyn Future<Output = Result<()>>>>),
    Rabbitmq(Pin<Box<dyn Future<Output = Result<()>>>>),
}
