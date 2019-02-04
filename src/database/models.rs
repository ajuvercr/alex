use super::schema::*;
use chrono::{NaiveDateTime};

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
pub struct Post {
    pub id: i32,
    pub uuid: i32,
    pub title: String,
    pub body: String,
    pub created: NaiveDateTime,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
pub struct User {
    pub id: i32,
    pub uuid: i32,
    pub name: String,
    pub email: String,
    pub password_hash: i64,
}

#[derive(Queryable, AsChangeset, Debug)]
pub struct Topic {
    pub post_id: i32,
    pub user_id: i32,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Insertable, Debug)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password_hash: i64,
    pub uuid: i32,
}

joinable!(topics -> users (user_id));
joinable!(topics -> posts (post_id));