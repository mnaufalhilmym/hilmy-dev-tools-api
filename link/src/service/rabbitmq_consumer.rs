use anyhow::{Error, Result};
use diesel::{
    query_dsl::methods::FilterDsl,
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, RunQueryDsl,
};
use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
    Channel,
};
use uuid::Uuid;

use crate::schema;

struct Rabbitmq {
    channel: Channel,
    queue: String,
}

pub struct Config {
    pub address: String,
    pub consumer_tag: String,
    pub queue: String,
    pub app_mode: String,
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
}

async fn init(config: &Config) -> Result<Rabbitmq> {
    let rabbitmq_connection: lapin::Connection = lapin::Connection::connect(
        &config.address,
        lapin::ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio),
    )
    .await?;
    let rabbitmq_channel = rabbitmq_connection.create_channel().await?;
    rabbitmq_channel
        .queue_declare(
            "delete.account",
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;
    Ok(Rabbitmq {
        channel: rabbitmq_channel,
        queue: config.queue.to_owned(),
    })
}

pub async fn consumer(config: Config) -> Result<()> {
    let rabbitmq = init(&config).await?;

    let rabbitmq_consumer = &mut rabbitmq
        .channel
        .basic_consume(
            &rabbitmq.queue,
            &config.consumer_tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    loop {
        if let Some(delivery) = rabbitmq_consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    delivery.ack(BasicAckOptions::default()).await?;

                    let payloads = Uuid::from_slice(&delivery.data)?;

                    let db_conn = &mut tools_lib_db::pg::connection::get_connection(
                        &config.app_mode,
                        &config.db_pool,
                    )
                    .map_err(|e| Error::msg(e.to_string()))?;

                    diesel::delete(
                        schema::link::table.filter(schema::link::created_by_id.eq(payloads)),
                    )
                    .execute(db_conn)?;
                }
                Err(e) => eprintln!("Failed to consume queue message: {e}"),
            }
        }
    }
}
