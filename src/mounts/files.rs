

use rocket::{Rocket};
use rocket::response::{Response, NamedFile};
use rocket::http::ContentType;

use std::path::{Path, PathBuf};

#[get("/wasm/<file..>")]
fn wasm<'a>(file: PathBuf) -> Option<Response<'a>> {
    match NamedFile::open(Path::new("wasm/").join(file)) {
        Ok(body) => Some(Response::build()
            .header(ContentType::new("application", "wasm"))
            .streamed_body(body)
            .finalize()),
        Err(_) => None
    }
}

#[get("/style/<file..>")]
fn style(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("style/").join(file)).ok()
}

pub fn fuel(r: Rocket) -> Rocket {
    r.mount("/", routes![wasm, style])
}