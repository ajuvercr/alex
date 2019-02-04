#![feature(plugin, async_await, await_macro, futures_api, proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate error_chain;

#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate serde_json;

extern crate chrono;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[macro_use] extern crate tera;

#[macro_use] extern crate diesel;

extern crate base64;
extern crate rand;

// extern crate eva;
extern crate config;
extern crate futures;
extern crate bytes;
extern crate futures_fs;

extern crate ws;

use rocket::{State};
use rocket::request::Form;
use rocket::response::{Redirect};
use rocket::http::Cookies;

use std::path::PathBuf;

mod upload;
mod dairy;
mod files;
// mod my_eva;

pub use self::errors::*;

pub mod auth;
pub mod util;
pub mod template;
pub mod database;
use template::Template;

use self::util::Context;

pub mod errors {
    use rocket::response::{self, Responder};
    use rocket::{Request};
    use crate::template::Template;
    
    use crate::util::Context;

    error_chain! {
        types {
            Error, ErrorKind, ResultExt, Result;
        }

        errors {
            TemplateError(context: Context, template: &'static str, reason: &'static str) {
                description("Error happend in a template")
                display("{}", reason)
            }

            AuthError {
                description("Username not found in database")
                display("Username not found")
            }
        }

        foreign_links {
            JsonError(serde_json::Error);
            IOError(std::io::Error);
        }
    }


    impl<'r> Responder<'r> for Error {
        fn respond_to(self, x: &Request) -> response::Result<'r> {
            let mut errors = Vec::new();
            let mut error_list = self.iter();

            if let Some(e) = error_list.next() {
                errors.push(format!("Error: {}", e));
            }

            for e in error_list {
                errors.push(format!("caused by: {}", e));
            }

            if let ErrorKind::TemplateError(c, t, _) = self.kind() {
                let c = c.clone().insert("errors", errors);
                Template::render(t.clone(), &c.inner()).respond_to(x)
            } else {
                let context = Context::new().insert("errors", errors);
                Template::render("error", &context.inner()).respond_to(x)
            }
        }
    }
}

#[get("/logout")]
fn logout(user: auth::Auth, auth: State<auth::AuthState>) -> Result<Redirect> {
    auth.invalidate_token(&user.username)?;
    Ok(Redirect::to("/"))
}

#[get("/")]
fn secure_root(user: auth::Auth) -> Result<Template> {
    let context = Context::new().insert("username", user.username.clone());

    Ok(Template::render("diary", &context.inner()))
}

#[get("/", rank = 2)]
fn root() -> Result<Template> {
    let context = Context::new();

    Ok(Template::render("index", &context.inner()))
}

#[get("/signup")]
fn signup_red() -> Redirect {
    Redirect::to("/")
}

#[post("/signup", data="<signup>")]
fn signup(mut cookies: Cookies, signup: Form<auth::Signup>, auth: State<auth::AuthState>) -> Result<Redirect> {
    let signup: auth::Signup = signup.into_inner();
    
    auth.add_user(signup, &mut cookies).chain_err(|| ErrorKind::TemplateError(Context::new(), "index", "Cannot add user to database"))?;

    Ok(Redirect::to("/"))
}

#[get("/login")]
fn login_red() -> Redirect {
    Redirect::to("/")
}

#[post("/login", data="<signup>")]
fn login(mut cookies: Cookies, signup: Form<auth::Signup>, auth: State<auth::AuthState>) -> Result<Redirect> {
    let signup: auth::Signup = signup.into_inner();

    auth.auth_user(signup, &mut cookies).chain_err(|| ErrorKind::TemplateError(Context::new(), "index", "Incorrect login combination"))?;

    Ok(Redirect::to("/"))
}

#[get("/<_file..>", rank = 3)]
fn catch_all(_file: PathBuf) -> Result<Template> {
    let context = Context::new().insert("errors", vec!["Please Log In First"]);

    Ok(Template::render("index", &context.inner()))
}

fn rocket() -> rocket::Rocket {
    let rocket = rocket::ignite();
    let rocket = upload::fuel(rocket);
    let rocket = dairy::fuel(rocket);
    let rocket = files::fuel(rocket);
    // let rocket = my_eva::fuel(rocket);

    rocket.mount("/", routes![root, secure_root, logout, signup, login, login_red, signup_red, catch_all])
}

fn main() -> Result<()> {
    rocket()
        .manage(auth::AuthState::new()?)
        .attach(template::TemplateFairing)
        .attach(database::DbConn::fairing())
    .launch();

    Ok(())
}