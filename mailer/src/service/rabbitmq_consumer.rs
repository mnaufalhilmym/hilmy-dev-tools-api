use std::sync::Arc;

use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
    Channel, Result,
};
use lettre::{message::MessageBuilder, SmtpTransport};
use tools_mailer::contract::MailReq;

use crate::helper;

struct Rabbitmq {
    channel: Channel,
    queue: String,
}

#[derive(Clone)]
pub struct Config {
    pub address: String,
    pub queue: String,
}

async fn init(config: &Config) -> Result<Rabbitmq> {
    let rabbitmq_connection = lapin::Connection::connect(
        &config.address,
        lapin::ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio),
    )
    .await?;
    let rabbitmq_channel = rabbitmq_connection.create_channel().await?;
    rabbitmq_channel
        .queue_declare(
            "mailer",
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;
    Ok(Rabbitmq {
        channel: rabbitmq_channel,
        queue: config.queue.to_owned(),
    })
}

pub async fn mailer(
    config: Config,
    message_builder: MessageBuilder,
    smtp_transport: SmtpTransport,
) -> Result<()> {
    let rabbitmq = init(&config).await?;

    let consumer = &mut rabbitmq
        .channel
        .basic_consume(
            &rabbitmq.queue,
            "consumer_tag",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    loop {
        if let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    delivery.ack(BasicAckOptions::default()).await?;

                    let payloads: Vec<MailReq> = serde_json::from_slice(&delivery.data)
                        .map_err(|e| lapin::Error::IOError(Arc::new(e.into())))?;

                    for payload in payloads {
                        helper::mailer::send_mail(&message_builder, &smtp_transport, payload)
                            .map_err(|e| {
                                lapin::Error::IOError(Arc::new(std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    e,
                                )))
                            })?;
                    }
                }
                Err(e) => eprintln!("Failed to consume queue message: {e}"),
            }
        }
    }
}
