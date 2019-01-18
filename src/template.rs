use crate::util::Context;

use tera::{Tera, Result};
use serde_json::value::{Value, to_value};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = compile_templates!("templates/*");
        tera.autoescape_on(vec![".sql"]);
        tera.register_filter("do_nothing", do_nothing_filter);
        println!("build {:?}", 
        tera.build_inheritance_chains()
        );
        tera
    };
}

pub struct Test;

impl Test {
    pub fn test() {
        println!("{:?}", TEMPLATES);
    }
}

use std::collections::{HashMap, BTreeMap};
pub fn do_nothing_filter(value: Value, _: HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(&s).unwrap())
}

pub struct Template {
    name: String,
    context: BTreeMap<String, Value>,
}

impl Template {
    pub fn render(file: &str, context: &BTreeMap<String, Value>) -> Self {
        Template {
            name: file.to_string(),
            context: context.clone(),
        }
    }
}

use rocket::request::{Request};
use rocket::response::{self, Response, Responder};
use std::io::Cursor;
impl<'r> Responder<'r> for Template {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        match TEMPLATES.render(&format!("{}", &self.name), &self.context) {
            Ok(s) => {
                Response::build()
                    .sized_body(Cursor::new(s))
                    .ok()
            },
            Err(e) => {
                println!("{:?}", e);
                Response::build()
                    .sized_body(Cursor::new("Errored"))
                    .ok()
            }
        }
    }
}