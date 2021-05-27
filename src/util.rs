use serde::Serialize;
use std::collections::BTreeMap;
use serde_json::Value;

use std::sync::{Arc, Mutex};
use rand::rngs::StdRng;

pub type Random = Arc<Mutex<StdRng>>;

#[derive(FromForm, Debug, Clone)]
pub struct Signup {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "0")]
pub enum TopicID {
    New(String),
    Exist(String),
}

#[derive(Deserialize, Debug, Clone)]
pub struct DairyEntry {
    pub title: String,
    pub synopsis: Option<String>,
    pub content: String,
    pub topics: Vec<TopicID>,
}

impl Signup {
    pub fn username(&self) -> String {
        self.username.clone()
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Context {
    inner: BTreeMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            inner: BTreeMap::new()
        }
    }

    pub fn insert<T>(self, at: &str, value: T) -> Self 
        where T: Serialize {
            let mut c = self.inner;
            c.insert(String::from(at), serde_json::to_value(value).unwrap());
            Context {
                inner: c
            }
    }

    pub fn inner(&self) -> BTreeMap<String, Value> {
        self.inner.clone()
    }

    pub fn merge(self, other: Context) -> Context {
        Context {
            inner: self.inner.into_iter().chain(other.inner.into_iter()).collect()
        }
    }
}

use rocket::http::RawStr;
use rocket::request::FromFormValue;
impl<'v> FromFormValue<'v> for Context {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        println!("{:?}", form_value);
        match form_value.parse::<usize>() {
            Ok(age) if age >= 21 => Ok(Context::new()),
            _ => Ok(Context::new()),
        }
    }
}

use crate::errors::{self, ResultExt};
use std::process::{Command, Stdio};
use std::io::{Write};
pub fn from_markdown(s: String) -> errors::Result<String> {
    let mut child = Command::new("/usr/bin/pandoc")
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn()
                            .chain_err(|| "Failed to spawn process")?;

    child.stdin.as_mut().unwrap().write_all(s.as_bytes()).chain_err(|| "Could not write to child")?;
    child.wait_with_output().chain_err(|| "Child didn't finish well")
        .and_then(|s| String::from_utf8(s.stdout).chain_err(|| "Couldn't convert to String"))
}