use super::schema::*;
use chrono::{NaiveDateTime};

pub type ID = i32;
pub type UUID = i32;

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
pub struct Post {
    pub id: ID,
    pub uuid: UUID,
    pub title: String,
    pub synopsis: Option<String>,
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
    pub uuid: UUID,
    pub title: &'a str,
    pub synopsis: Option<String>,
    pub body: &'a str,
    pub created: NaiveDateTime,
}

use crate::util::{DairyEntry, Signup};
use chrono::offset::Utc;
impl<'a> NewPost<'a> {
    pub fn from_dairy_entry(entry: &'a DairyEntry, id: UUID) -> Self {
        let time = Utc::now().naive_utc();

        NewPost {
            uuid: id,
            title: &entry.title,
            synopsis: entry.synopsis.clone(),
            body: &entry.content,
            created: time,
        }
    }
}

#[derive(Insertable, Debug)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password_hash: i64,
    pub uuid: UUID,
}

impl<'a> NewUser<'a> {
    pub fn from_signup(user: &'a Signup<i64>, id: UUID) -> Self {
        NewUser {
            name: &user.username, 
            email: "blank",
            uuid: id,
            password_hash: user.password,
        }
    }
}

joinable!(topics -> users (user_id));
joinable!(topics -> posts (post_id));