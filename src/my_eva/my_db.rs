
use std::path::{PathBuf, Path};
use std::sync::Mutex;
use std::collections::HashMap;

use chrono::Duration;
use chrono::offset::Utc;
use chrono::prelude::*;

use std::fs;

use serde_json;

use eva::{NewTask, Task};
use eva::database::Database;
use eva::Result;
use eva::errors::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MNewTask {
    pub content: String,
    pub deadline: i32,
    pub duration: i32,
    pub importance: i32,
}

impl From<&MNewTask> for NewTask {
    fn from(task: &MNewTask) -> NewTask {
        let naive_deadline = NaiveDateTime::from_timestamp(i64::from(task.deadline), 0);
        let deadline = Utc.from_utc_datetime(&naive_deadline);
        let duration = Duration::seconds(i64::from(task.duration));
        NewTask {
            content: task.content.clone(),
            deadline: deadline,
            duration: duration,
            importance: task.importance as u32,
        } 
    }
}

impl From<&NewTask> for MNewTask {
    fn from(task: &NewTask) -> MNewTask {
        MNewTask {
            content: task.content.clone(),
            deadline: task.deadline.timestamp() as i32,
            duration: task.duration.num_seconds() as i32,
            importance: task.importance as i32,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MTask {
    pub id: i32,
    pub content: String,
    pub deadline: i32,
    pub duration: i32,
    pub importance: i32,
}

impl From<&MTask> for Task {
    fn from(task: &MTask) -> Task {
        let naive_deadline = NaiveDateTime::from_timestamp(i64::from(task.deadline), 0);
        let deadline = Utc.from_utc_datetime(&naive_deadline);
        let duration = Duration::seconds(i64::from(task.duration));
        Task {
            id: task.id as u32,
            content: task.content.clone(),
            deadline: deadline,
            duration: duration,
            importance: task.importance as u32,
        }
    }
}

impl From<&Task> for MTask {
    fn from(task: &Task) -> MTask {
        MTask {
            id: task.id as i32,
            content: task.content.clone(),
            deadline: task.deadline.timestamp() as i32,
            duration: task.duration.num_seconds() as i32,
            importance: task.importance as i32,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MyDB {
    state: Mutex<HashMap<u32, MTask>>,
    loc: PathBuf,
    id: usize,
}

impl MyDB {
    pub fn new<P>(location: P) -> MyDB
        where
            P: AsRef<Path> + Clone,
            std::path::PathBuf: std::convert::From<P> {
        let state = fs::read_to_string(location.clone()).map_err(|_| ())
            .and_then(|s| {
                serde_json::from_str(&s).map_err(|_| ())
            })
            .unwrap_or(HashMap::new());
        
        MyDB {
            id: state.len(),
            loc: PathBuf::from(location),
            state: Mutex::new(state),
        }
    }
}

impl Drop for MyDB {
    fn drop(&mut self) {
        // TODO make async?
        if let Ok(string) = serde_json::to_string(self) {
            if let Err(_) = fs::write(self.loc.clone(), string) {
                println!("Couldn't save DB to file :o");
            }
        } else {
            println!("Couldn't serialize DB :o");
        }
    }
}

impl Database for MyDB {
    fn add_task<'a: 'b, 'b>(&'a self, task: NewTask) -> LocalFutureObj<'b, Result<Task>> {
        let future_task = async move {
            let id = 0;
            let task = Task {
                id,
                content: task.content,
                deadline: task.deadline,
                duration: task.duration,
                importance: task.importance,
            };
            let mut state = self.state.lock().unwrap();
            state.insert(id, MTask::from(&task));

            Ok(task)
        };
        LocalFutureObj::new(Box::new(future_task))
    }

    fn remove_task<'a: 'b, 'b>(&'a self, id: u32) -> LocalFutureObj<'b, Result<()>> {
        let future_task = async move {
            let mut state = self.state.lock().unwrap();
            state.remove(&id);

            Ok(())
        };
        LocalFutureObj::new(Box::new(future_task))
    }

    fn find_task<'a: 'b, 'b>(&'a self, id: u32) -> LocalFutureObj<'b, Result<Task>> {
        let future_task = async move {
            let mut state = self.state.lock().unwrap();

            let out = state.get(&id).and_then(|x| Some(Task::from(x))).chain_err(|| ErrorKind::Database("while trying to find a task".to_owned()));
            out
        };
        
        LocalFutureObj::new(Box::new(future_task))
    }

    fn update_task<'a: 'b, 'b>(&'a self, task: Task) -> LocalFutureObj<'b, Result<()>> {
        let future_task = async move {
            let mut state = self.state.lock().unwrap();

            state.insert(task.id, MTask::from(&task));
            Ok(())
        };
        LocalFutureObj::new(Box::new(future_task))
    }

    fn all_tasks<'a: 'b, 'b>(&'a self) -> LocalFutureObj<'b, Result<Vec<Task>>> {
        let future_task = async move {
            let mut state = self.state.lock().unwrap();

            Ok(state.values().map(|t| Task::from(t)).collect())
        };
        LocalFutureObj::new(Box::new(future_task))
    }
}