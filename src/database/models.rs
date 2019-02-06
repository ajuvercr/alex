use super::schema::*;
use chrono::{NaiveDateTime};

pub type ID = i32;
pub type UUID = i32;

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
pub struct Post {
    pub id: ID,
    pub uuid: UUID,
    pub title: String,
    pub body: String,
    pub created: NaiveDateTime,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
pub struct User {
    pub id: ID,
    pub uuid: UUID,
    pub email: String,
    pub name: String,
    pub password_hash: i64,
}

#[derive(Queryable, AsChangeset, Debug)]
pub struct Topic {
    pub post_id: ID,
    pub user_id: ID,
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
    pub uuid: UUID,
}

joinable!(topics -> users (user_id));
joinable!(topics -> posts (post_id));