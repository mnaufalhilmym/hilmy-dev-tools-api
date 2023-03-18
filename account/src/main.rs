use std::error::Error;

use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use rdkafka::ClientConfig;
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
    let kafka_addrs = env::Env::kafka_addrs();

    let db_pool = tools_lib_db::pg::connection::create_connection_pool(&database_url);
    let db_conn = &mut tools_lib_db::pg::connection::get_connection(&app_mode, &db_pool).unwrap();
    tools_lib_db::pg::migration::run_migrations(db_conn, MIGRATIONS)?;

    let redis_pool = tools_lib_db::redis::connection::create_connection_pool(&redis_url);

    let kafka_producer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_addrs)
        .set("message.timeout.ms", "500")
        .create()?;

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
            },
        ))
        .serve(service_addrs.parse()?)
        .await?;

    Ok(())
}
