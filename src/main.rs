#![feature(plugin, async_await, await_macro, futures_api, proc_macro_hygiene, decl_macro)]
#![recursion_limit="4096"]

#[macro_use] extern crate error_chain;

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate chrono;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

extern crate tera;

#[macro_use] extern crate diesel;

extern crate base64;
extern crate rand;

// extern crate eva;
extern crate config;
extern crate futures;
extern crate bytes;
extern crate futures_fs;

extern crate ws;

extern crate bcrypt;

use std::sync::{Arc, Mutex};
use std::path::{PathBuf, Path};
use rocket::response::{NamedFile, Redirect};
use rand::prelude::*;

mod mounts;
// mod my_eva;

pub use self::errors::*;

pub mod auth;
pub mod util;
pub mod template;
pub mod database;

use self::util::Context;
use self::template::Template;

#[allow(deprecated)]
pub mod errors;

#[get("/")]
fn secure_root(_user: auth::Auth) -> Result<Redirect> {
    Ok(Redirect::to("/diaries"))
}

#[get("/", rank = 2)]
fn root() -> Result<Template> {
    Ok(Template::render("index", &Context::new().inner()))
}

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("favicon.ico")).ok()
}

#[get("/<_file..>", rank = 3)]
fn catch_all(_file: PathBuf) -> Result<Template> {
    let context = Context::new().insert("errors", vec!["Please Log In First"]);

    Ok(Template::render("index", &context.inner()))
}

fn rocket() -> rocket::Rocket {
    let rocket = rocket::ignite();
    let rocket = mounts::fuel(rocket);
    // let rocket = my_eva::fuel(rocket);

    rocket.mount("/", routes![root, secure_root, favicon, catch_all])
}

fn main() -> Result<()> {
    rocket()
        .manage(auth::AuthState::new()?)
        .manage(Arc::new(Mutex::new(StdRng::seed_from_u64(rand::random::<u64>()))))
        .attach(template::TemplateFairing)
        .attach(database::DbConn::fairing())
    .launch();

    Ok(())
}