use rocket_sync_db_pools::{database, diesel};

#[database("blog")]
pub struct BlogDBConn(diesel::PgConnection);
