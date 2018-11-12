#![feature(plugin, async_await, await_macro, futures_api)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

extern crate serde;
extern crate serde_json;

extern crate chrono;

extern crate rocket;

extern crate rocket_contrib;

extern crate base64;

extern crate eva;
use eva::database::Database;

extern crate config;

use std::sync::Mutex;
use std::path::PathBuf;

mod dairy;
mod upload;
mod files;
mod my_eva;

fn rocket() -> rocket::Rocket {
    let rocket = rocket::ignite();
    let rocket = upload::fuel(rocket);
    let rocket = dairy::fuel(rocket);
    let rocket = files::fuel(rocket);
    let rocket = my_eva::fuel(rocket);

    rocket
}

fn main() {
    rocket().launch();
}