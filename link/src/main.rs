use std::error::Error;

use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use env::UseMsgBroker;
use service::{kafka_consumer, rabbitmq_consumer, Consumer};

mod controller;
mod env;
mod grpc_server;
mod model;
mod proto;
mod schema;
mod service;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let app_name = env::Env::app_name();
    let app_mode = env::Env::app_mode();
    let service_name = env::Env::service_name();
    let service_addrs = env::Env::service_addrs();
    let database_url = env::Env::database_url();
    let use_msg_broker = UseMsgBroker::from_string(env::Env::use_msg_broker()).unwrap();
    let consume = env::Env::msg_broker_consume();

    let db_pool = tools_lib_db::pg::connection::create_connection_pool(&database_url);
    let db_conn = &mut tools_lib_db::pg::connection::get_connection(&app_mode, &db_pool).unwrap();
    tools_lib_db::pg::migration::run_migrations(db_conn, MIGRATIONS)?;

    let consumer = match use_msg_broker {
        UseMsgBroker::Kafka => {
            let kafka_addrs = env::Env::kafka_addrs();
            let kafka_group_id = env::Env::kafka_group_id();
            let kafka_consumer_config = kafka_consumer::Config {
                brokers: kafka_addrs,
                group_id: kafka_group_id,
                input_topic: consume,
                app_mode: app_mode.clone(),
                db_pool: db_pool.clone(),
            };
            Consumer::Kafka(Box::pin(kafka_consumer::consumer(kafka_consumer_config)))
        }
        UseMsgBroker::Rabbitmq => {
            let rabbitmq_addrs = env::Env::rabbitmq_addrs();
            let rabbitmq_consumer_tag = env::Env::rabbitmq_consumer_tag();
            let rabbitmq_consumer_config = rabbitmq_consumer::Config {
                address: rabbitmq_addrs,
                consumer_tag: rabbitmq_consumer_tag,
                queue: consume,
                app_mode: app_mode.clone(),
                db_pool: db_pool.clone(),
            };
            Consumer::Rabbitmq(Box::pin(rabbitmq_consumer::consumer(
                rabbitmq_consumer_config,
            )))
        }
    };

    println!("{app_name} {service_name} is running on {service_addrs} in {app_mode}.");

    tokio::try_join!(
        match consumer {
            Consumer::Kafka(future) => future,
            Consumer::Rabbitmq(future) => future,
        },
        grpc_server::serve(grpc_server::Config {
            app_mode,
            db_pool,
            service_addrs,
        })
    )
    .unwrap();

    Ok(())
}
