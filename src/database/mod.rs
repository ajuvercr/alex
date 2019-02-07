use crate::errors::*;

pub mod models;
pub mod schema;

pub use models::{ID, UUID, NewPost, NewUser};

use schema::{users, posts, topics};

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

pub fn add_user<'a>(user: NewUser, conn: &DbConn) -> Result<models::User> {
    diesel::insert_into(users::table)
        .values(&user)
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

type WithTopic<T> = Eq<topics::name, T>;
pub fn with_topic_name<T>(name: T) -> WithTopic<T>
where
    T: AsExpression<Text>,
{
    topics::name.eq(name)
}

pub fn add_post(post: NewPost, conn: &DbConn) -> Result<models::Post> {
    diesel::insert_into(posts::table)
        .values(&post)
        .get_result(&conn.0)
        .chain_err(|| "Could not add post to DB!")
}

pub fn get_posts(conn: &DbConn) -> Result<Vec<models::Post>> {
    posts::table
        .load::<models::Post>(&conn.0)
        .chain_err(|| "Could not get posts from DB!")
}