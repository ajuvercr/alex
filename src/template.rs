use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Rocket, State};
use rocket::response::{Response, Responder, self};

use tera::{Tera};
use std::io::Cursor;

use std::collections::BTreeMap;
use serde_json::Value;
type Context = BTreeMap<String, Value>;

pub struct Template {
    name: String,
    context: Context,
}

impl Template {
    pub fn render(template: &str, context: &Context) -> Self {
        Template {
            name: format!("{}.html.tera", template),
            context: context.clone(),
        }
    }
}

use rocket::outcome::Outcome;
impl<'r> Responder<'r> for Template {

    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match req.guard::<State<Tera>>() {
            Outcome::Success(tera) => {
                let s = match tera.render(&self.name, &self.context) {
                    Ok(s) => s,
                    Err(e) => {
                        println!("{:?}", e);
                        String::from("Something failed")
                    }
                };
                Response::build()
                    .sized_body(Cursor::new(s))
                    .ok()
            },
            _ => {
                Response::build()
                    .sized_body(Cursor::new(format!("EVERYTHING FAILED")))
                    .ok()
            },
        }
    }
    
}

pub struct TemplateFairing;

impl Fairing for TemplateFairing {
    fn info(&self) -> Info {
        Info {
            name: "Custom Tera Templates",
            kind: Kind::Attach
        }
    }

    fn on_attach(&self, rocket: Rocket) -> std::result::Result<Rocket, Rocket> {
        match Tera::new("templates/**/*") {
            Ok(t) => Ok(rocket.manage(t)),
            Err(e) => {
                println!("{:?}", e);
                Err(rocket)
            }
        }
    }
}


use tera::*;
fn make_url_for(urls: BTreeMap<String, String>) -> GlobalFn {
    Box::new(move |args| -> tera::Result<Value> {
        match args.get("name") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) =>  Ok(to_value(urls.get(&v).unwrap()).unwrap()),
                Err(_) => Err("oops".into()),
            },
            None => Err("oops".into()),
        }
    })
}

