use std::error::Error;

use diesel::r2d2::{Pool, PooledConnection};
use redis::Client;

pub type RedisPool = Pool<Client>;
pub type RedisPooled = PooledConnection<Client>;

pub fn create_connection_pool(redis_url: &str) -> RedisPool {
    let manager = Client::open(redis_url).unwrap();

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .unwrap()
}

pub fn get_connection(
    app_mode: &str,
    pool: &RedisPool,
) -> Result<RedisPooled, Box<dyn Error + Send + Sync>> {
    match pool.get() {
        Ok(conn) => Ok(conn),
        Err(e) => Err(match app_mode {
            "DEBUG" => format!("Failed to get Redis db: {e}").into(),
            _ => format!("Failed to get Redis db").into(),
        }),
    }
}
