use rocket::{Rocket, State};
use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::Redirect;

use crate::errors::*;
use crate::auth;
use crate::util::Context;
use crate::database::{DbConn, self};

#[get("/logout")]
fn logout(user: auth::Auth, auth: State<auth::AuthState>) -> Result<Redirect> {
    auth.invalidate_token(&user.username)?;
    Ok(Redirect::to("/"))
}

#[get("/signup")]
fn signup_red() -> Redirect {
    Redirect::to("/")
}

#[post("/signup", data="<signup>")]
fn signup(mut cookies: Cookies, signup: Form<auth::Signup>, auth: State<auth::AuthState>, conn: DbConn) -> Result<Redirect> {
    let signup: auth::Signup = signup.into_inner();
    
    let u = database::add_user(&signup.username, &signup.password, 0, &conn);
    println!("{:?}", u);
    auth.add_user(signup, &mut cookies).chain_err(|| ErrorKind::TemplateError(Context::new(), "index", "Cannot add user to database"))?;

    Ok(Redirect::to("/"))
}

#[get("/login")]
fn login_red() -> Redirect {
    Redirect::to("/")
}

#[post("/login", data="<signup>")]
fn login(mut cookies: Cookies, signup: Form<auth::Signup>, auth: State<auth::AuthState>, conn: DbConn) -> Result<Redirect> {
    let signup: auth::Signup = signup.into_inner();

    println!("{:?}", database::get_users(&conn));
    auth.auth_user(signup, &mut cookies).chain_err(|| ErrorKind::TemplateError(Context::new(), "index", "Incorrect login combination"))?;

    Ok(Redirect::to("/"))
}

pub fn fuel(r: Rocket) -> Rocket {
    r.mount("/", routes![logout, signup, signup_red, login, login_red])
}
