
use error_chain::ChainedError;
// use futures::executor::block_on;

use serde_json;
use rocket::{Rocket, State};
use rocket_contrib::json::Json;

use eva::configuration::{SchedulingStrategy, Configuration};

pub mod my_db;
use self::my_db::MyDB as DB;

mod errors {
    error_chain! {
        errors {
            Read(what: String) {
                description("configuration parsing error")
                display("An error occurred while trying to read {}", what)
            }
            FileCreation(config_name: String) {
                description("file creation error while reading configuration")
                display("I could not create {}", config_name)
            }
            ShellExpansion(what: String) {
                description("shell expansion error while reading configuration")
                display("An error occurred while trying to expand the configuration of {}", what)
            }
            DatabaseConnect(path: String) {
                description("database connection error")
                display("I could not connect to the database ({})", path)
            }
            Default(what: String) {
                description("setting defaults error while reading configuration")
                display("An error occurred while trying to set the default configuration of {}",
                        what)
            }
        }
    }
}

pub fn fuel(r: Rocket) -> Rocket {
    let mut db = DB::new("HI");
    r.manage(get_conf(db)).mount("/", routes![get, new_task])
}

fn get_conf(db: DB) -> Configuration {
    Configuration {
        database: Box::new(db),
        scheduling_strategy: SchedulingStrategy::Importance,
    }
}


#[get("/eva")]
pub fn get(s: State<Configuration>) -> serde_json::Result<String> {
    // let ts: Vec<my_db::MTask> = block_on(eva::schedule(&s, "importance".to_string())).unwrap().iter().map(|t| my_db::MTask::from(t)).collect();
    // serde_json::to_string(&ts)
    Ok("nope".to_string())
}

// '{"content": "hallo", "deadline": 10, "duration": 20, "importance": 1}'
#[post("/eva", data="<data>")]
pub fn new_task(data: Json<my_db::MNewTask>, s: State<Configuration>) -> String {
    println!("data {:?}", data);
    // block_on(eva::add(&s, eva::NewTask::from(&data.into_inner())));
    "Success".to_string()
}