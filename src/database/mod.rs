use crate::errors::*;

pub mod models;
pub mod schema;

pub use models::{ID, UUID};

use schema::{users, posts, topics};

use crate::util::Signup;

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::dsl::{Eq};
use diesel::expression::AsExpression;

#[database("sqlite_logs")]
pub struct DbConn(diesel::PgConnection);

pub fn users() -> users::table {
    users::table
}

pub fn posts() -> posts::table {
    posts::table
}

pub fn topics() -> topics::table {
    topics::table
}

// pub fn joined<'a>() -> SelectStatement<impl QuerySource> {
//     topics::table
//         .inner_join(posts::table).inner_join(users::table)
// }


type WithUserUUID<T> = Eq<users::uuid, T>;

pub fn with_user_uuid<T>(uuid: T) -> WithUserUUID<T>
where
    T: AsExpression<Integer>,
{
    users::uuid.eq(uuid)
}

type WithUserName<T> = Eq<users::name, T>;

pub fn with_user_name<T>(name: T) -> WithUserName<T>
where
    T: AsExpression<Text>,
{
    users::name.eq(name)
}

pub fn add_user<'a>(user: &Signup<i64>, uuid: i32, conn: &DbConn) -> Result<models::User> {
    let new_user = models::NewUser {
        name: &user.username, 
        email: "blank",
        uuid: uuid,
        password_hash: user.password,
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

pub fn get_user_with_name(name: &str, conn: &DbConn) -> Result<models::User> {
    users::table
        .filter(with_user_name(name))
        .get_result(&conn.0)
        .chain_err(|| "No such user")
}