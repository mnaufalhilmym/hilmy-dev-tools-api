use actix_cors::Cors;
use actix_web::{middleware::Logger, web::ServiceConfig, App, HttpServer};
use controller::register;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use std::io::{Error, ErrorKind, Result};

use crate::{controller::CtxData, env::AppMode};

mod contract;
mod controller;
mod env;
mod model;
mod schema;
mod service;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[actix_web::main]
async fn main() -> Result<()> {
    let app_name = env::Env::app_name();
    let app_mode = env::Env::app_mode();
    let service_name = env::Env::service_name();
    let service_addrs = env::Env::service_addrs();
    let database_url = env::Env::database_url();

    let db_conn_pool = tools_db::pg::connection::create_connection_pool(&database_url);
    let db_conn = &mut tools_db::pg::connection::get_connection(&app_mode, &db_conn_pool).unwrap();
    if let Err(e) = tools_db::pg::migration::run_migrations(db_conn, MIGRATIONS) {
        eprintln!("Error running migrations: {e}");
        return Err(Error::new(ErrorKind::Other, e));
    };

    println!("{app_name} {service_name} is running on {service_addrs} in {app_mode}.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_method()
                    .allow_any_origin()
                    .allow_any_header()
                    .expose_any_header(),
            )
            .configure(|service_config: &mut ServiceConfig| {
                register(
                    service_config,
                    CtxData {
                        app_mode: AppMode::from(app_mode.to_owned()),
                        db_conn_pool: db_conn_pool.to_owned(),
                    },
                )
            })
    })
    .bind(service_addrs)?
    .run()
    .await
}
