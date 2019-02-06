use rocket::{Rocket, State};
use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::Redirect;

use crate::errors::*;
use crate::auth;
use crate::util::{Context, Signup};
use crate::database::{DbConn, self};

#[get("/logout")]
fn logout(cookies: Cookies, auth: State<auth::AuthState>) -> Result<Redirect> {
    let uuid = cookies.get("uuid").and_then(|u| u.value().parse().ok());
    if let Some(uuid) = uuid {
        auth.invalidate_token(uuid)?;
    }
    Ok(Redirect::to("/"))
}

#[get("/signup")]
fn signup_red() -> Redirect {
    Redirect::to("/")
}

#[post("/signup", data="<signup>")]
fn signup(mut cookies: Cookies, signup: Form<Signup<String>>, auth: State<auth::AuthState>, conn: DbConn) -> Result<Redirect> {
    let signup: Signup<String> = signup.into_inner();
    
    // let u = database::add_user(&signup.username, &signup.password, 0, &conn);
    // println!("{:?}", u);
    auth.add_user(signup.hashed(), &mut cookies, &conn).chain_err(|| ErrorKind::TemplateError(Context::new(), "index", "Cannot add user to database"))?;

    Ok(Redirect::to("/"))
}

#[get("/login")]
fn login_red() -> Redirect {
    Redirect::to("/")
}

#[post("/login", data="<signup>")]
fn login(mut cookies: Cookies, signup: Form<Signup<String>>, auth: State<auth::AuthState>, conn: DbConn) -> Result<Redirect> {
    let signup: Signup<String> = signup.into_inner();

    println!("{:?}", database::get_users(&conn));
    // println!("{:?}",
    //     database::joined()
    //         .filter(database::with_user_UUID(10))
    //         .load::<(models::Topic, models::Post, models::User)>(&conn.0)
    //         .expect("couldn't load users")
    // );
    auth.auth_user(signup.hashed(), &mut cookies, &conn).chain_err(|| ErrorKind::TemplateError(Context::new(), "index", "Incorrect login combination"))?;

    Ok(Redirect::to("/"))
}

pub fn fuel(r: Rocket) -> Rocket {
    r.mount("/", routes![logout, signup, signup_red, login, login_red])
}
