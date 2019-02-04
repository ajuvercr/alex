
pub mod models;
pub mod schema;

#[database("sqlite_logs")]
pub struct DbConn(diesel::PgConnection);