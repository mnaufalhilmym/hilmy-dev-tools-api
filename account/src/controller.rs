use tools_db::pg::connection::DbPool;

mod account;

pub struct AccountController {
    pub app_mode: String,
    pub db_conn_pool: DbPool,
}
