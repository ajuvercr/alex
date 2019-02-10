
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
pub fn dairy(data: Json<DairyEntry>, user: auth::Auth, conn: database::DbConn, rand: rocket::State<Random>) -> Result<Redirect> {
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
pub fn get(user: auth::Auth, conn: database::DbConn) -> Result<Template> {
    let topics: Vec<String> = database::get_topics(&conn).unwrap_or(Vec::new()).iter().map(|x| x.name.clone()).collect();

    let c = Context::new()
        .insert("username", user.uuid)
        .insert("topics", topics);

    Ok(Template::render("diary", &c.inner()))
}

pub fn fuel(r: Rocket) -> Rocket {
    use std::thread;
    use std::process::{Command, Stdio};
    use std::io::{Write};

    thread::spawn(move || {
        ws::listen("127.0.0.1:3012", |out| {
            println!("token {:?}", out.token());
            move |msg| {
                if let ws::Message::Text(s) = msg {
                    let mut child = Command::new("/usr/bin/pandoc")
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn()                      
                            .ok().expect("failed to spawn process");

                    child.stdin.as_mut().unwrap().write_all(s.as_bytes()).expect("Could not write to child");
                    let output = child.wait_with_output().unwrap();
                    out.send(String::from_utf8(output.stdout).unwrap())
                } else {
                    out.send("please send text")
                }
            }
        }).unwrap();
    });

    r.mount("/", routes![get, dairy])
}