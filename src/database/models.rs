use super::schema::*;
use chrono::{NaiveDateTime};

pub type ID = i32;
pub type UUID = i32;

#[derive(Serialize, Queryable, Identifiable, AsChangeset, Debug)]
pub struct Post {
    pub id: ID,
    pub uuid: UUID,
    pub title: String,
    pub synopsis: Option<String>,
    pub body: String,
    pub created: NaiveDateTime,
}
#[derive(Debug, Serialize, Clone)]
pub struct PostWithTopics {
    pub uuid: UUID,
    pub title: String,
    pub synopsis: Option<String>,
    pub body: String,
    pub topics: Vec<String>,
}

impl PostWithTopics {
    pub fn new(p: &Post) -> Self {
        PostWithTopics {
            uuid: p.uuid,
            title: p.title.clone(),
            synopsis: p.synopsis.clone(),
            body: p.body.clone(),
            topics: Vec::new(),
        }
    }

    pub fn add_topic(&mut self, t: &Topic) {
        self.topics.push(t.name.clone())
    }
}

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
pub struct User {
    pub id: ID,
    pub uuid: UUID,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
pub struct Topic {
    pub id: ID,
    pub name: String,
}

#[derive(Insertable, Queryable, AsChangeset, Debug)]
#[table_name="post_topics"]
pub struct PostTopic {
    pub post_id: ID,
    pub topic_id: ID,
}

#[derive(Insertable, Queryable, AsChangeset, Debug)]
#[table_name="user_posts"]
pub struct UserPost {
    pub user_id: ID,
    pub post_id: ID,
}

#[derive(Insertable, Queryable, AsChangeset, Debug)]
#[table_name="user_topics"]
pub struct UserTopic {
    user_id: ID,
    topic_id: ID,
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
    pub password_hash: String,
    pub uuid: UUID,
}

impl<'a> NewUser<'a> {
    pub fn from_signup(user: &'a Signup, password: String, id: UUID) -> Self {

        NewUser {
            name: &user.username,
            email: "blank",
            uuid: id,
            password_hash: password,
        }
    }
}

#[derive(Insertable, Debug)]
#[table_name="topics"]
pub struct NewTopic<'a> {
    pub name: &'a str,
}

impl<'a> From<&'a str> for NewTopic<'a> {
    fn from(name: &'a str) -> Self {
        NewTopic {
            name
        }
    }
}
