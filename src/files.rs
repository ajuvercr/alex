

use rocket::{Route, Rocket};
use rocket::Outcome;
use rocket::response::{Response, Redirect, NamedFile};
use rocket::request::{self, Request, FromRequest};
use rocket::http::ContentType;

use std::path::{Path, PathBuf};

pub struct ApiKey;

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let uri = request.uri().to_string();
        if let Some(ext) = Path::new(&uri).extension() {
            if ext == "wasm" {
                return Outcome::Success(ApiKey);
            }
        }

        Outcome::Forward(())
    }
}

#[get("/<file..>", rank = 1)]
fn wasm_files<'a>(file: PathBuf, _key: ApiKey) -> Option<Response<'a>> {
    match NamedFile::open(Path::new("WWW/").join(file)) {
        Ok(body) => Some(Response::build()
            .header(ContentType::new("application", "wasm"))
            .streamed_body(body)
            .finalize()),
        Err(_) => None
    }
}

#[get("/<file..>", rank = 2)]
fn files(file: PathBuf) -> Option<NamedFile> {
    // is file a file or a directory?
    NamedFile::open(Path::new("WWW/").join(file)).ok()
}

#[get("/", rank = 3)]
fn home() -> Redirect {
    Redirect::to("/index.html")
}

pub fn fuel(r: Rocket) -> Rocket {
    r.mount("/", routes![wasm_files, files, home])
}