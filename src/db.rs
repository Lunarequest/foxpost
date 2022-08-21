use rocket_sync_db_pools::{database, diesel};

#[database("blog")]
struct BlogDBConn(diesel::PgConnection);