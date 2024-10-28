use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use tonic::transport::Server;

use crate::{controller::LinkController, proto};

pub struct Config {
    pub app_mode: String,
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
    pub service_addrs: String,
}

pub async fn serve(config: Config) -> Result<()> {
    Server::builder()
        .add_service(proto::link::LinkServiceServer::new(LinkController {
            app_mode: config.app_mode,
            db_pool: config.db_pool,
        }))
        .serve(config.service_addrs.parse()?)
        .await?;
    Ok(())
}
