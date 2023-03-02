use std::error::Error;

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

pub type DbConnMgr = ConnectionManager<PgConnection>;
pub type DbPool = Pool<DbConnMgr>;
pub type DbPooled = PooledConnection<DbConnMgr>;

pub fn create_connection_pool(database_url: &str) -> DbPool {
    let manager = DbConnMgr::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .unwrap()
}

pub fn get_connection(
    app_mode: &str,
    pool: &DbPool,
) -> Result<DbPooled, Box<dyn Error + Send + Sync>> {
    match pool.get() {
        Ok(conn) => Ok(conn),
        Err(e) => Err(match app_mode {
            "DEBUG" => format!("Failed to get db: {e}").into(),
            _ => format!("Failed to get db").into(),
        }),
    }
}
