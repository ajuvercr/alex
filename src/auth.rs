use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

use futures_fs::FsPool;

use rocket::request::{self, Request, FromRequest, State};
use rocket::outcome::IntoOutcome;
use rocket::http::{Cookie, Cookies};

use bytes::Bytes;
use futures::Future;
use std::thread;
use futures::sink::Sink;

use crate::errors::*;

pub struct Randomiser {
    r: Arc<Mutex<StdRng>>,
}

impl Randomiser {
    fn new() -> Randomiser {
        Randomiser {
            r: Arc::new(Mutex::new(StdRng::seed_from_u64(rand::random::<u64>()))),
        }
    }

    fn random(& self) -> Result<u64> {
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

        cookies
            .get_private("token")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|token|
                cookies.get("username").map(|username| (username.value(), token))
            )
            .and_then(|(username, token)|
                db.validate_token(username, token).ok().map(|is_ok| (username.to_string(), is_ok))
            )
            .and_then(|(username, is_ok)| if is_ok { Some(Auth{username}) } else { None })
            .or_forward(())
    }
}

pub type Token = u64;

#[derive(FromForm, Debug, Clone)]
pub struct Signup {
    pub username: String,
    pub password: String,
}

impl Signup {
    fn username(&self) -> String {
        self.username.clone()
    }
}

pub struct AuthState {
    db: Arc<Mutex<HashMap<String, String>>>,
    tokens: Arc<Mutex<HashMap<String, (Instant, Token)>>>,
    r: Randomiser,
    fs: FsPool
}

impl AuthState {
    pub fn new() -> Result<AuthState> {
        Ok(
            AuthState {
                db: Arc::new(Mutex::new(Self::load().unwrap_or(HashMap::new()))),
                tokens: Arc::new(Mutex::new(HashMap::new())),
                r: Randomiser::new(),
                fs: FsPool::default(),
            }
        )
    }

    pub fn add_user(&self, user: Signup, cookies: &mut Cookies) -> Result<()> {
        match self.db.lock() {
            Ok(mut state) => {
                if state.contains_key(&user.username) {
                    bail!("Username already in use!");
                } else {
                    state.insert(user.username.clone(), user.password.clone());
                }
            },
            Err(_) => bail!("Could not lock state!"),
        }

        let token = self.r.random()?;

        match self.tokens.lock() {
            Ok(mut state) => {
                state.insert(user.username(), (Instant::now(), token));
            },
            Err(_) => bail!("Could not lock state!"),
        }

        cookies.add_private(Cookie::new("token", token.to_string()));
        cookies.add(Cookie::new("username", user.username));

        self.save().chain_err(|| "Could not save db!")?;
        Ok(())
    }

    pub fn auth_user(&self, user: Signup, cookies: &mut Cookies) -> Result<()> {
        match self.db.lock() {
            Ok(state) => {
                let pw = state.get(&user.username).ok_or(ErrorKind::AuthError)?;
                if pw != &user.password {
                    bail!("Incorrect Password");
                }
            },
            Err(_) => bail!("Could not lock state!"),
        }

        let token = self.r.random()?;

        match self.tokens.lock() {
            Ok(mut state) => {
                state.insert(user.username(), (Instant::now(), token));
            },
            Err(_) => bail!("Could not lock state!"),
        }
        
        cookies.add_private(Cookie::new("token", token.to_string()));
        cookies.add(Cookie::new("username", user.username));

        self.save().chain_err(|| "Could not save db!")?;
        Ok(())
    }

    pub fn validate_token(&self, username: &str, token: Token) -> Result<bool> {
        match self.tokens.lock() {
            Ok(state) => {
                if let Some((at, t)) = state.get(username) {
                    return Ok(*t == token && Instant::now().duration_since(*at) < Duration::from_secs(600));
                }
            },
            Err(_) => bail!("Could not lock state!"),
        }
        Ok(false)
    }

    pub fn invalidate_token(&self, username: &String) -> Result<()> {
        match self.tokens.lock() {
            Ok(mut state) => {
                state.remove(username);
            },
            Err(_) => bail!("Could not lock state!"),
        }
        Ok(())
    }

    fn load() -> Result<HashMap<String, String>> {
        use std::fs;
        let state = fs::read_to_string("db.json")?;

        Ok(serde_json::from_str(&state)?)
    }
    fn get_sink(&self) -> impl Sink<SinkItem=Bytes, SinkError=io::Error> + Sized {
        self.fs.write("db.json", Default::default())
    }

    fn save(&self) -> Result<()> {
        let state = match self.db.lock() {
            Ok(state) => state.clone(),
            Err(_) => bail!("Could not lock state!"),
        };

        let state = serde_json::to_string(&state)?;
        // <[i32]>::get(self, index).map(|v| v as &Any)


        let ss = self.get_sink()
                .send(Bytes::from(state.as_bytes()));

        thread::spawn(move || {
            if let Err(e) = ss.wait() {
                println!("Error: {:?}", e);
            }
            println!("Saved");
        });

        Ok(())
    }
}

use std::io;

impl Drop for AuthState {
    fn drop(&mut self) {
        println!("dropping");
        self.save().expect("Could not save");
    }
}