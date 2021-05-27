
use rocket::Rocket;
use rocket::response::Redirect;

use rocket_contrib::json::Json;
use rand::Rng;

use crate::errors::*;
use crate::auth;
use crate::util::{Context, DairyEntry, TopicID, Random};
use crate::template::Template;
use crate::database::{self, models};

use diesel::prelude::*;


#[post("/diary", format = "json", data="<data>")]
fn dairy(data: Json<DairyEntry>, user: auth::Auth, conn: database::DbConn, rand: rocket::State<Random>) -> Result<Redirect> {
    let mut rand = rand.lock().unwrap();
    let data: DairyEntry = data.into_inner();
    println!("new dairy entry {:?}", data);

    let entry = models::NewPost::from_dairy_entry(&data, rand.gen());
    let post = database::add_post(entry, user.uuid, &conn)?;

    let topics: Vec<models::Topic> = data.topics.iter().filter_map(|t| {
        match t {
            TopicID::New(t) => database::add_topic(models::NewTopic::from(t.as_str()), &conn).ok(),
            TopicID::Exist(t) => database::topics()
                .filter(database::with_topic_name(t))
                .get_result(&conn.0).ok()
        }
    }).collect();

    database::link_topics_to_post(&topics, &post, &conn)?;

    Ok(Redirect::to("/diary"))
}

#[get("/diary")]
fn get(user: auth::Auth, conn: database::DbConn) -> Result<Template> {
    let topics: Vec<String> = database::get_topics(&conn).unwrap_or(Vec::new()).iter().map(|x| x.name.clone()).collect();

    let c = Context::new()
        .insert("username", user.username)
        .insert("topics", topics);

    Ok(Template::render("diary", &c.inner()))
}

#[get("/diaries")]
fn list(user: auth::Auth, conn: database::DbConn) -> Result<Template> {
    let topics: Vec<String> = database::get_topics(&conn).unwrap_or(Vec::new()).iter().map(|x| x.name.clone()).collect();

    let posts: Option<Vec<models::PostWithTopics>> = database::get_posts(&conn).ok();
    let c = Context::new()
        .insert("username", user.username)
        .insert("topics", topics)
        .insert("posts", posts);

    Ok(Template::render("diaryList", &c.inner()))
}

pub fn fuel(r: Rocket) -> Rocket {
    use std::thread;
    use crate::util::from_markdown;

    thread::spawn(move || {
        ws::listen("127.0.0.1:3012", |out| {
            println!("token {:?}", out.token());
            move |msg| {
                if let ws::Message::Text(s) = msg {
                    if let Ok(s) = from_markdown(s) {
                        out.send(s)
                    } else {
                        println!("failed to encode to html");
                        Ok(())
                    }
                } else {
                    out.send("please send text")
                }
            }
        }).unwrap();
    });

    r.mount("/", routes![get, dairy, list])
}