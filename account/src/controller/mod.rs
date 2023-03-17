use rdkafka::producer::FutureProducer;
use tools_lib_db::{pg::connection::DbPool, redis::connection::RedisPool};

mod account;

pub struct AccountController {
    pub app_mode: String,
    pub db_pool: DbPool,
    pub redis_pool: RedisPool,
    pub argon2_hash_secret: String,
    pub jwt_secret: String,
    pub kafka_producer: FutureProducer,
}
