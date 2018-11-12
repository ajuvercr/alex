
use rocket_contrib::Json;
use rocket::Route;
use rocket::Rocket;

use chrono::Local;
use std::io;
use std::io::Write;
use std::path::{Path};
use std::fs::{File, create_dir_all};

#[derive(Deserialize)]
pub struct DairyEntry {
    title: String,
    content: String,
}

#[post("/diary", data="<data>")]
pub fn dairy(data: Json<DairyEntry>) -> io::Result<String> {
    let local = Local::now();
    let date: String = local.format("%Y/%m_%b/%d").to_string();
    let file_name = format!("{}.md", data.title);
    let path = Path::new("./dairy").join(date).join(file_name);

    create_dir_all(path.parent().unwrap())?;
    let mut f = File::create(path)?;
    f.write_all(data.content.as_bytes())?;

    Ok("Success".to_string())
}

pub fn fuel(r: Rocket) -> Rocket {
    r.mount("/", routes![dairy])
}