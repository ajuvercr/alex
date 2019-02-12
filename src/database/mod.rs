use crate::errors::*;

pub mod models;
pub mod schema;

pub use models::{ID, UUID, NewPost, NewUser, NewTopic};

use schema::{users, posts, topics, post_topics, user_posts};

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



pub fn add_post(post: NewPost, owner: UUID, conn: &DbConn) -> Result<models::Post> {
    let post: models::Post = diesel::insert_into(posts::table)
        .values(&post)
        .get_result(&conn.0)
        .chain_err(|| "Could not add post to DB!")?;

    let owner: models::User = users::table
        .filter(with_user_uuid(owner))
        .get_result(&conn.0)
        .chain_err(|| "Couldn't get owner from DB!")?;

    let link = models::UserPost {
        user_id: owner.id,
        post_id: post.id,
    };

    diesel::insert_into(user_posts::table)
        .values(&link)
        .execute(&conn.0)?;

    Ok(post)
}

pub fn link_topics_to_post(topics: &Vec<models::Topic>, post: &models::Post, conn: &DbConn) -> Result<()> {
    let ts: Vec<models::PostTopic> = topics.iter().map(|t| models::PostTopic {
        post_id: post.id,
        topic_id: t.id,
    }).collect();

    diesel::insert_into(post_topics::table)
        .values(&ts)
        .execute(&conn.0)?;

    Ok(())
}

use std::collections::HashMap;

pub fn get_posts(conn: &DbConn) -> Result<Vec<models::PostWithTopics>> {
    let posts = post_topics::table.inner_join(posts::table).inner_join(topics::table)
        .load::<(models::PostTopic, models::Post, models::Topic)>(&conn.0)
        .chain_err(|| "Couldn't get posts from DB!")?;

    let mut postmap: HashMap<UUID, models::PostWithTopics> = HashMap::new();
    
    posts.iter().for_each(|(_, post, topic)| {
        if let Some(p) = postmap.get_mut(&post.uuid) {
            p.add_topic(topic);
        } else {
            let mut pwt = models::PostWithTopics::new(post);
            pwt.add_topic(topic);
            postmap.insert(post.uuid, pwt);
        }
    });

    Ok(postmap.values().cloned().collect())
}

type WithTopic<T> = Eq<topics::name, T>;
pub fn with_topic_name<T>(name: T) -> WithTopic<T>
where
    T: AsExpression<Text>,
{
    topics::name.eq(name)
}


pub fn add_topic(post: NewTopic, conn: &DbConn) -> Result<models::Topic> {
    diesel::insert_into(topics::table)
        .values(&post)
        .get_result(&conn.0)
        .chain_err(|| "Could not add post to DB!")
}

pub fn get_topics(conn: &DbConn) -> Result<Vec<models::Topic>> {
    topics::table
        .load::<models::Topic>(&conn.0)
        .chain_err(|| "Could not get posts from DB!")
}