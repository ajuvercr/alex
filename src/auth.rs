use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

use rocket::request::{self, Request, FromRequest, State};
use rocket::outcome::{IntoOutcome};
use rocket::http::{Cookie, Cookies};

use crate::errors::*;
use crate::database::{DbConn, self, UUID};
use crate::util::Signup;

pub struct Randomiser {
    r: Arc<Mutex<StdRng>>,
}

impl Randomiser {
    fn new() -> Randomiser {
        Randomiser {
            r: Arc::new(Mutex::new(StdRng::seed_from_u64(rand::random::<u64>()))),
        }
    }

    fn random<T>(& self) -> Result<T>
        where rand::distributions::Standard: rand::distributions::Distribution<T> {
        if let Ok(mut ss) = self.r.lock() {
            Ok(ss.gen())
        } else {
            bail!("Could not lock state")
        }
    }
}

pub struct Auth {
    pub username: String
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let db = request.guard::<State<AuthState>>()?;
        let mut cookies = request.cookies();

        let token = cookies.get_private("token").and_then(|t| {
            println!("{:?}", t);
            t.value().parse().ok()
        });
        let uuid = cookies.get("uuid").and_then(|u| u.value().parse().ok());

        println!("{:?} {:?}", token, uuid);
        let combo = 
        if let (Some(token), Some(uuid)) = (token, uuid) {
                Some((uuid, token))
        } else {
            None
        };

        combo.and_then(|(uuid, token)|
                db.validate_token(uuid, token).ok().map(|is_ok| (uuid.to_string(), is_ok))
            )
            .and_then(|(username, is_ok)| if is_ok { Some(Auth{username}) } else { None })
            .or_forward(())
    }
}

pub type Token = u32;

pub struct AuthState {
    tokens: Arc<Mutex<HashMap<UUID, (Instant, Token)>>>,
    r: Randomiser,
}

impl AuthState {
    pub fn new() -> Result<AuthState> {
        Ok(
            AuthState {
                tokens: Arc::new(Mutex::new(HashMap::new())),
                r: Randomiser::new(),
            }
        )
    }

    pub fn add_user(&self, user: Signup<i64>, cookies: &mut Cookies, conn: &DbConn) -> Result<()> {
        if database::get_user_with_name(&user.username, conn).is_ok() {
            bail!("Username already in use!");
        }

        let uuid = database::add_user(&user, self.r.random()?, &conn)?.uuid;
        let token = self.r.random()?;

        match self.tokens.lock() {
            Ok(mut state) => {
                state.insert(uuid, (Instant::now(), token));
            },
            Err(_) => bail!("Could not lock state!"),
        }

        cookies.add_private(Cookie::new("token", token.to_string()));
        cookies.add(Cookie::new("uuid", uuid.to_string()));

        Ok(())
    }

    pub fn auth_user(&self, user: Signup<i64>, cookies: &mut Cookies, conn: &DbConn) -> Result<()> {
        let user_db = database::get_user_with_name(&user.username, &conn)?;
        if user_db.password_hash != user.password {
            bail!("Incorrect Password");
        }

        let token = self.r.random()?;

        match self.tokens.lock() {
            Ok(mut state) => {
                state.insert(user_db.uuid, (Instant::now(), token));
            },
            Err(_) => bail!("Could not lock state!"),
        }
        
        cookies.add_private(Cookie::new("token", token.to_string()));
        cookies.add(Cookie::new("uuid", user_db.uuid.to_string()));

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