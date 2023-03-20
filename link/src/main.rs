use tonic::transport::Server;

mod controller;
mod env;
mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let app_name = env::Env::app_name();
    let app_mode = env::Env::app_mode();
    let service_name = env::Env::service_name();
    let service_addrs = env::Env::service_addrs();
    let database_url = env::Env::database_url();

    let db_pool = tools_lib_db::pg::connection::create_connection_pool(&database_url);
    let db_conn = &mut tools_lib_db::pg::connection::get_connection(&app_mode, &db_pool).unwrap();
    tools_lib_db::pg::migration::run_migrations(db_conn, MIGRATIONS)?;

    println!("{app_name} {service_name} is running on {service_addrs} in {app_mode}.");

    Server::builder()
        .add_service(proto::link::LinkServiceServer::new(LinkController {
            app_mode,
            db_pool,
        }))
        .serve(service_addrs.parse()?)
        .await?;

    Ok(())
}
