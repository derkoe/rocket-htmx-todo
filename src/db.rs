use rocket_sync_db_pools::{database, diesel};

#[database("postgres_database")]
pub struct Conn(diesel::PgConnection);
