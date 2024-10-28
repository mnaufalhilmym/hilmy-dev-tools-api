use std::str::FromStr;

use anyhow::{Error, Result};
use diesel::{
    query_dsl::methods::FilterDsl,
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, RunQueryDsl,
};
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    ClientConfig, Message,
};
use uuid::Uuid;

use crate::schema;

#[derive(Clone)]
pub struct Config {
    pub brokers: String,
    pub group_id: String,
    pub input_topic: String,
    pub app_mode: String,
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
}

pub async fn consumer(config: Config) -> Result<()> {
    let kafka_consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &config.group_id)
        .set("bootstrap.servers", &config.brokers)
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()?;

    kafka_consumer.subscribe(&[&config.input_topic])?;

    loop {
        match kafka_consumer.recv().await {
            Ok(message) => {
                let payloads = message.payload_view::<str>();
                if let Some(payloads) = payloads {
                    if let Ok(payloads) = payloads {
                        let payloads = Uuid::from_str(payloads)?;

                        let db_conn = &mut tools_lib_db::pg::connection::get_connection(
                            &config.app_mode,
                            &config.db_pool,
                        )
                        .map_err(|e| Error::msg(e.to_string()))?;

                        diesel::delete(
                            schema::link::table.filter(schema::link::created_by_id.eq(payloads)),
                        )
                        .execute(db_conn)?;

                        if let Err(e) = kafka_consumer.commit_message(&message, CommitMode::Async) {
                            eprintln!("Kafka commit message error: {e}");
                        };
                    }
                }
            }
            Err(e) => eprintln!("Failed to consume queue message: {e}"),
        }
    }
}
