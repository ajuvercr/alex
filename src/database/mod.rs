
pub mod models;
pub mod schema;
use schema::{users, posts};

use diesel::RunQueryDsl;

#[database("sqlite_logs")]
pub struct DbConn(diesel::PgConnection);

pub fn add_user<'a>(name: &'a str, email: &'a str, pw_hash: i64, conn: &DbConn) -> models::User {
    let new_user = models::NewUser {
        name, email,
        password_hash: pw_hash,
        uuid: 10
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&conn.0)
        .expect("Error saving new user")
}

pub fn get_users(conn: &DbConn) -> Vec<models::User> {
    
    users::table
        .load::<models::User>(&conn.0)
        .expect("Error loading posts")
}