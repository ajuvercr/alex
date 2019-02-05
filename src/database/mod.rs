
use crate::errors::*;

pub mod models;
pub mod schema;
use schema::{users, posts};

use diesel::RunQueryDsl;

#[database("sqlite_logs")]
pub struct DbConn(diesel::PgConnection);

pub fn add_user<'a>(name: &'a str, email: &'a str, pw_hash: i64, conn: &DbConn) -> Result<models::User> {
    let new_user = models::NewUser {
        name, email,
        password_hash: pw_hash,
        uuid: 10
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&conn.0)
        .chain_err(|| "Could not add user to DB!")
}

pub fn get_users(conn: &DbConn) -> Result<Vec<models::User>> {
    users::table
        .load::<models::User>(&conn.0)
        .chain_err(|| "Could not get users from DB!")
}