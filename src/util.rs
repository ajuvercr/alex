use serde::Serialize;
use std::collections::BTreeMap;
use serde_json::Value;

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
