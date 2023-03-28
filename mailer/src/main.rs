use std::error::Error;

use futures::{stream::FuturesUnordered, StreamExt};
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};

use crate::service::{kafka_consumer, rabbitmq_consumer};

mod contract;
mod env;
mod helper;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let app_name = env::Env::app_name();
    let app_mode = env::Env::app_mode();
    let service_name = env::Env::service_name();
    let use_msg_broker = env::Env::use_msg_broker();
    let consume = env::Env::msg_broker_consume();
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

    let mut kafka_consumer_config: Option<kafka_consumer::Config> = None;
    let mut rabbitmq_consumer_config: Option<rabbitmq_consumer::Config> = None;
    if use_msg_broker.is_kafka() {
        let kafka_addrs = env::Env::kafka_addrs();
        let kafka_group_id = env::Env::kafka_group_id();
        kafka_consumer_config = Some(kafka_consumer::Config {
            brokers: kafka_addrs,
            group_id: kafka_group_id,
            input_topic: consume,
        })
    } else if use_msg_broker.is_rabbitmq() {
        let rabbitmq_addrs = env::Env::rabbitmq_addrs();
        rabbitmq_consumer_config = Some(rabbitmq_consumer::Config {
            address: rabbitmq_addrs,
            queue: consume,
        })
    }

    println!("{app_name} {service_name} is running in {app_mode}.");

    let num_workers = std::thread::available_parallelism()?.get();
    if use_msg_broker.is_kafka() {
        (0..num_workers)
            .map(|_| {
                tokio::spawn(kafka_consumer::mailer(
                    kafka_consumer_config.to_owned().unwrap(),
                    message_builder.to_owned(),
                    smtp_transport.to_owned(),
                ))
            })
            .collect::<FuturesUnordered<_>>()
            .for_each(|res| async {
                match res {
                    Ok(res) => {
                        if let Err(e) = res {
                            eprintln!("{}", e);
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                }
            })
            .await;
    } else if use_msg_broker.is_rabbitmq() {
        (0..num_workers)
            .map(|_| {
                tokio::spawn(rabbitmq_consumer::mailer(
                    rabbitmq_consumer_config.to_owned().unwrap(),
                    message_builder.to_owned(),
                    smtp_transport.to_owned(),
                ))
            })
            .collect::<FuturesUnordered<_>>()
            .for_each(|res| async {
                match res {
                    Ok(res) => {
                        if let Err(e) = res {
                            eprintln!("{}", e);
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                }
            })
            .await;
    }

    Ok(())
}
