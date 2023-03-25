use std::io::Result;

use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::dto::AppData;

mod contract;
mod controller;
mod dto;
mod env;
mod static_file;

#[actix_web::main]
async fn main() -> Result<()> {
    let app_name = env::Env::app_name();
    let app_mode = env::Env::app_mode();
    let service_name = env::Env::service_name();
    let service_addrs = env::Env::service_addrs();
    let service_gql_addrs = env::Env::service_gql_addrs();
    let site_link_url = env::Env::site_link_url();

    println!("{app_name} {service_name} is running on {service_addrs} in {app_mode}.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppData {
                app_mode: app_mode.to_owned(),
                gql_addrs: service_gql_addrs.to_owned(),
                site_link_url: site_link_url.to_owned(),
            }))
            .service(controller::root::root)
            .service(controller::resolver::resolve_link)
    })
    .bind(service_addrs)?
    .run()
    .await
}
