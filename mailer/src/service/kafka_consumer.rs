use std::error::Error;

use lettre::{message::MessageBuilder, SmtpTransport};
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    ClientConfig, Message,
};
use tools_mailer::contract::MailReq;

use crate::helper;

#[derive(Clone)]
pub struct Config {
    pub brokers: String,
    pub group_id: String,
    pub input_topic: String,
}

pub async fn mailer(
    config: Config,
    message_builder: MessageBuilder,
    smtp_transport: SmtpTransport,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &config.group_id)
        .set("bootstrap.servers", &config.brokers)
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()?;

    consumer.subscribe(&[&config.input_topic])?;

    loop {
        match consumer.recv().await {
            Ok(message) => {
                let payloads = message.payload_view::<str>();
                if let Some(payloads) = payloads {
                    if let Ok(payloads) = payloads {
                        let payloads: Vec<MailReq> = serde_json::from_str(payloads)?;
                        for payload in payloads {
                            helper::mailer::send_mail(&message_builder, &smtp_transport, payload)?;
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
