use tools_lib_db::pg::connection::DbPool;

mod link;

pub struct LinkController {
    pub app_mode: String,
    pub db_pool: DbPool,
}
