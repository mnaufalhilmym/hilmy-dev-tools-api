use std::error::Error;

use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use tonic::transport::Server;

use crate::controller::AccountController;

mod controller;
mod dto;
mod env;
mod helper;
mod model;
mod proto;
mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let app_name = env::Env::app_name();
    let app_mode = env::Env::app_mode();
    let service_name = env::Env::service_name();
    let service_addrs = env::Env::service_addrs();
    let database_url = env::Env::database_url();
    let redis_url = env::Env::redis_url();
    let argon2_hash_secret = env::Env::argon2_hash_secret();
    let jwt_secret = env::Env::jwt_secret();
    let use_msg_broker = env::Env::use_msg_broker();

    let db_pool = tools_lib_db::pg::connection::create_connection_pool(&database_url);
    let db_conn = &mut tools_lib_db::pg::connection::get_connection(&app_mode, &db_pool).unwrap();
    tools_lib_db::pg::migration::run_migrations(db_conn, MIGRATIONS)?;

    let redis_pool = tools_lib_db::redis::connection::create_connection_pool(&redis_url);

    let mut kafka_producer: Option<rdkafka::producer::FutureProducer> = None;
    let mut rabbitmq_channel: Option<lapin::Channel> = None;
    if use_msg_broker.is_kafka() {
        let kafka_addrs = env::Env::kafka_addrs();
        let kafka_msg_timeout = env::Env::kafka_msg_timeout();
        kafka_producer = Some(
            rdkafka::ClientConfig::new()
                .set("bootstrap.servers", &kafka_addrs)
                .set("message.timeout.ms", &kafka_msg_timeout)
                .create()?,
        );
    } else if use_msg_broker.is_rabbitmq() {
        let rabbitmq_addrs = env::Env::rabbitmq_addrs();
        let rabbitmq_connection = lapin::Connection::connect(
            &rabbitmq_addrs,
            lapin::ConnectionProperties::default()
                .with_executor(tokio_executor_trait::Tokio::current())
                .with_reactor(tokio_reactor_trait::Tokio),
        )
        .await
        .unwrap();
        rabbitmq_channel = Some(rabbitmq_connection.create_channel().await.unwrap());
        rabbitmq_channel
            .as_ref()
            .unwrap()
            .confirm_select(lapin::options::ConfirmSelectOptions::default())
            .await
            .unwrap();
        rabbitmq_channel
            .as_ref()
            .unwrap()
            .queue_declare(
                "mailer",
                lapin::options::QueueDeclareOptions::default(),
                lapin::types::FieldTable::default(),
            )
            .await
            .unwrap();
    }

    println!("{app_name} {service_name} is running on {service_addrs} in {app_mode}.");

    Server::builder()
        .add_service(proto::account::AccountServiceServer::new(
            AccountController {
                app_mode,
                db_pool,
                redis_pool,
                argon2_hash_secret,
                jwt_secret,
                kafka_producer,
                rabbitmq_channel,
            },
        ))
        .serve(service_addrs.parse()?)
        .await?;

    Ok(())
}
