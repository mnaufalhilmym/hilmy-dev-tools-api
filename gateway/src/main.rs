use std::{
    fs,
    io::{Error, ErrorKind, Result},
};

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::ServiceConfig, App, HttpServer};
use controller::register;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

use crate::{
    controller::CtxData,
    env::{AppMode, AppName, ServiceName},
};

mod contract;
mod controller;
mod dto;
mod env;
mod gql_schema;
mod helper;
mod model;
mod schema;
mod service;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[actix_web::main]
async fn main() -> Result<()> {
    let app_name = AppName::from(env::Env::app_name());
    let app_mode = AppMode::from(env::Env::app_mode());
    let service_name = ServiceName::from(env::Env::service_name());
    let service_addrs = env::Env::service_addrs();
    let database_url = env::Env::database_url();
    let grpc_connect_timeout = env::Env::grpc_connect_timeout();

    let db_pool = tools_lib_db::pg::connection::create_connection_pool(&database_url);
    let db_conn =
        &mut tools_lib_db::pg::connection::get_connection(app_mode.as_str(), &db_pool).unwrap();
    if let Err(e) = tools_lib_db::pg::migration::run_migrations(db_conn, MIGRATIONS) {
        eprintln!("Error running migrations: {e}");
        return Err(Error::new(ErrorKind::Other, e));
    };

    let gql_schema = gql_schema::schema::build_gql_schema(gql_schema::schema::GqlData {
        app_mode: app_mode.to_owned(),
        db_pool,
        grpc_connect_timeout: grpc_connect_timeout.parse().unwrap(),
    });
    if app_mode.is_debug() {
        fs::write(
            "/app/gateway/schema/gateway.schema.graphql",
            &gql_schema.sdl(),
        )?;
    }

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
                        app_name: app_name.to_owned(),
                        app_mode: app_mode.to_owned(),
                        service_name: service_name.to_owned(),
                        gql_schema: gql_schema.to_owned(),
                    },
                )
            })
    })
    .bind(service_addrs)?
    .run()
    .await
}
