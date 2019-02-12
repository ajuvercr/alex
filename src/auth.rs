use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use rand::prelude::*;
use rand::rngs::StdRng;

use rocket::request::{self, Request, FromRequest, State};
use rocket::outcome::{IntoOutcome};
use rocket::http::{Cookie, Cookies};

use bcrypt::{hash, verify};

use crate::errors::*;
use crate::database::{DbConn, self, UUID, NewUser};
use crate::util::Signup;

pub struct Auth {
    pub uuid: UUID,
    pub username: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let db = request.guard::<State<AuthState>>()?;
        let mut cookies = request.cookies();

        let token = cookies.get_private("token").and_then(|t| {
            t.value().parse().ok()
        });
        let uuid: Option<UUID> = cookies.get("uuid").and_then(|u| u.value().parse().ok());


        let combo = if let (Some(token), Some(uuid)) = (token, uuid) {
                let username = cookies.get("username").map(|u| u.value().to_string()).unwrap_or(uuid.to_string());
                Some((uuid, token, username))
            } else {
                None
            };

        combo.and_then(|(uuid, token, username)|
                db.validate_token(uuid, token).ok().map(|is_ok| (uuid, username, is_ok))
            )
            .and_then(|(uuid, username, is_ok)| if is_ok { Some(Auth{uuid, username}) } else { None })
            .or_forward(())
    }
}

pub type Token = u32;

pub struct AuthState {
    tokens: Arc<Mutex<HashMap<UUID, (Instant, Token)>>>,
}

impl AuthState {
    pub fn new() -> Result<AuthState> {
        Ok(
            AuthState {
                tokens: Arc::new(Mutex::new(HashMap::new())),
            }
        )
    }

    pub fn add_user(&self, user: Signup, cookies: &mut Cookies, conn: &DbConn, rand: &mut StdRng) -> Result<()> {
        if database::get_user_with_name(&user.username, conn).is_ok() {
            bail!("Username already in use!");
        }

        let pw = hash(user.password.clone(), 8).chain_err(|| "Couldn't hash password")?;
        let uuid = database::add_user(NewUser::from_signup(&user, pw, rand.gen()), &conn)?.uuid;
        let token = rand.gen();

        match self.tokens.lock() {
            Ok(mut state) => {
                state.insert(uuid, (Instant::now(), token));
            },
            Err(_) => bail!("Could not lock state!"),
        }

        cookies.add_private(Cookie::new("token", token.to_string()));
        cookies.add(Cookie::new("uuid", uuid.to_string()));
        cookies.add(Cookie::new("username", user.username));


        Ok(())
    }

    pub fn auth_user(&self, user: Signup, cookies: &mut Cookies, conn: &DbConn, rand: &mut StdRng) -> Result<()> {
        let user_db = database::get_user_with_name(&user.username, &conn)?;
        if !verify(&user.password, &user_db.password_hash).chain_err(|| "Couldn't verify hash")? {
            bail!("Incorrect Password");
        }

        let token = rand.gen();

        match self.tokens.lock() {
            Ok(mut state) => {
                state.insert(user_db.uuid, (Instant::now(), token));
            },
            Err(_) => bail!("Could not lock state!"),
        }
        
        cookies.add_private(Cookie::new("token", token.to_string()));
        cookies.add(Cookie::new("uuid", user_db.uuid.to_string()));
        cookies.add(Cookie::new("username", user.username));

        Ok(())
    }

    pub fn validate_token(&self, uuid: UUID, token: Token) -> Result<bool> {
        match self.tokens.lock() {
            Ok(state) => {
                if let Some((at, t)) = state.get(&uuid) {
                    return Ok(*t == token && Instant::now().duration_since(*at) < Duration::from_secs(600));
                }
            },
            Err(_) => bail!("Could not lock state!"),
        }
        Ok(false)
    }

    pub fn invalidate_token(&self, uuid: UUID) -> Result<()> {
        match self.tokens.lock() {
            Ok(mut state) => {
                state.remove(&uuid);
            },
            Err(_) => bail!("Could not lock state!"),
        }
        Ok(())
    }
}