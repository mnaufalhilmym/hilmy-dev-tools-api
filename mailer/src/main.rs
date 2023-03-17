use std::error::Error;

use futures::{stream::FuturesUnordered, StreamExt};
use lettre::{
    message::MessageBuilder, transport::smtp::authentication::Credentials, SmtpTransport, Transport,
};
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    ClientConfig, Message,
};

use crate::contract::MailReq;

mod contract;
mod env;

async fn mailer(
    brokers: String,
    group_id: String,
    input_topic: String,
    message_builder: MessageBuilder,
    smtp_transport: SmtpTransport,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &group_id)
        .set("bootstrap.servers", &brokers)
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()?;

    consumer.subscribe(&[&input_topic])?;

    loop {
        match consumer.recv().await {
            Ok(message) => {
                let payloads = message.payload_view::<str>();
                if let Some(payloads) = payloads {
                    if let Ok(payloads) = payloads {
                        let payloads: Vec<MailReq> = serde_json::from_str(payloads)?;
                        for payload in payloads {
                            let message = message_builder
                                .to_owned()
                                .to(payload.to.parse()?)
                                .subject(payload.subject)
                                .body(payload.body)?;
                            smtp_transport.send(&message)?;
                        }
                    }
                }
                if let Err(e) = consumer.commit_message(&message, CommitMode::Async) {
                    eprintln!("Kafka commit message error: {e}");
                };
            }
            Err(e) => eprintln!("Kafka receive stream error: {e}"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let app_name = env::Env::app_name();
    let app_mode = env::Env::app_mode();
    let service_name = env::Env::service_name();
    let kafka_addrs = env::Env::kafka_addrs();
    let kafka_group_id = env::Env::kafka_group_id();
    let kafka_input_topic = env::Env::kafka_input_topic();
    let smtp_server = env::Env::smtp_server();
    let smtp_username = env::Env::smtp_username();
    let smtp_password = env::Env::smtp_password();
    let sender_name = env::Env::sender_name();
    let sender_email = env::Env::sender_email();

    let message_builder =
        lettre::Message::builder().from(format!("{sender_name} <{sender_email}>").parse().unwrap());

    let smtp_transport = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(Credentials::new(smtp_username, smtp_password))
        .build();

    println!("{app_name} {service_name} is running in {app_mode}.");

    let num_workers = std::thread::available_parallelism()?.get();
    (0..num_workers)
        .map(|_| {
            tokio::spawn(mailer(
                kafka_addrs.to_owned(),
                kafka_group_id.to_owned(),
                kafka_input_topic.to_owned(),
                message_builder.to_owned(),
                smtp_transport.to_owned(),
            ))
        })
        .collect::<FuturesUnordered<_>>()
        .for_each(|_| async { () })
        .await;

    Ok(())
}
