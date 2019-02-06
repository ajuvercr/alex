use serde::Serialize;
use std::collections::BTreeMap;
use serde_json::Value;

#[derive(FromForm, Debug, Clone)]
pub struct Signup<T> {
    pub username: String,
    pub password: T,
}

impl<T> Signup<T> {
    pub fn username(&self) -> String {
        self.username.clone()
    }
}

impl Signup<String> {
    pub fn hashed(&self) -> Signup<i64> {
        Signup {
            username: self.username(),
            password: calculate_hash(&self.password),
        }
    }
}

use std::hash::{Hash, Hasher};

fn calculate_hash<T: Hash>(t: &T) -> i64 {
    use std::collections::hash_map::DefaultHasher;
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    unsafe {
        std::mem::transmute::<u64, i64>(s.finish())
    }
}

#[derive(Clone, Serialize, Debug)]
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
