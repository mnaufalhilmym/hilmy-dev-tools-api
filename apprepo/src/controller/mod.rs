use tools_lib_db::pg::connection::DbPool;

mod apprepo;

pub struct ApprepoController {
    pub app_mode: String,
    pub db_pool: DbPool,
}
