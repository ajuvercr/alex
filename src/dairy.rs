
use rocket::Rocket;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use chrono::Local;
use std::io::Write;
use std::path::{Path};
use std::fs::{File, create_dir_all};

use crate::errors::*;
use crate::auth;
use crate::util::Context;

#[derive(FromForm, Debug, Clone)]
pub struct DairyEntry {
    title: String,
    content: String,
}

// TODO add real database
#[post("/diary", data="<data>")]
pub fn dairy(data: Form<DairyEntry>, user: auth::Auth) -> Result<Redirect> {
    let data: DairyEntry = data.into_inner();

    let local = Local::now();
    let date: String = local.format("%Y/%m_%b/%d").to_string();
    let file_name = format!("{}.md", data.title);
    let path = Path::new(&format!("./dairy/{}", user.username)).join(date).join(file_name);

    create_dir_all(path.parent().unwrap())?;
    let mut f = File::create(path)?;
    f.write_all(data.content.as_bytes())?;

    Ok(Redirect::to("/diary"))
}

#[get("/diary")]
pub fn get(user: auth::Auth) -> Result<Template> {
    let c = Context::new().insert("username", user.username);
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