
use rocket::response::{Response, NamedFile};
use rocket::{Rocket};

use std::io::{Write};
use std::fs::{File, create_dir_all, DirEntry, self};
use std::path::{Path, PathBuf};


use rocket::Request;
use rocket::response::{self, Responder};
use rocket_contrib::templates::Template;

use crate::util::Context;
use crate::auth;
use crate::errors::*;


#[post("/upload/<path..>", data = "<data>")]
pub fn post(data: Vec<u8>, path: PathBuf, user: auth::Auth) -> Result<String> {
    let mut path = Path::new(&format!("./upload/{}", user.username)).join(path);
    let pf = path.clone();
    
    create_dir_all(path.parent().unwrap())?;


    if path.exists() {
        println!("path exists!");
        let stem = pf.file_stem().and_then(|x| x.to_str()).unwrap_or("");
        let parent = pf.parent().unwrap_or(Path::new(""));
        let ext = pf.extension().and_then(|x| x.to_str()).unwrap_or("");
        let mut trie = 1;

        while path.exists() {
            path = parent.join(format!("{}({}).{}", stem, trie, ext));
            trie += 1;
        }
    }

    eprintln!("Uploading to {:?}", path);

    let mut f = File::create(path)?;
    f.write_all(&data)?;


    // match base64::decode(&data) {
    //     Ok(content) => {
    //         create_dir_all(path.parent().unwrap())?;
    //         let mut f = File::create(path.clone())?;
    //         f.write_all(&content)?
    //     },
    //     Err(e) => return Ok(format!("{{'Error': DecodeError: {} }}", e)),
    // };

    Ok("Success".to_string())
}

#[get("/upload")]
fn get_root<'r>(user: auth::Auth) -> Result<FileWrapper<'r>> {
    get(user, PathBuf::new())
}

fn is_file(d: &DirEntry) -> bool {
    d.file_type().map(|x| x.is_file()).unwrap_or(false)
}

pub enum FileWrapper<'r> {
    Template(Template),
    Response(Response<'r>),
}

impl<'r> Responder<'r> for FileWrapper<'r> {
    #[inline(always)]
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match self {
            FileWrapper::Template(t) => Response::build()
                .merge(t.respond_to(req)?)
                .ok(),
            FileWrapper::Response(r) => Response::build()
                .merge(r.respond_to(req)?)
                .ok()
        }
    }
}

pub fn get_context(user: auth::Auth, file: PathBuf) -> Result<Context> {

        let path_comps: Vec<String> = vec![String::from("upload")].iter().cloned().chain(file.iter().filter_map(|x| x.to_str()).map(|x| x.to_string())).collect();

        let path = Path::new(&format!("upload/{}", user.username)).join(file);

        if !path.exists() {
            println!("making dir {:?}", path);
            fs::create_dir_all(path.clone()).chain_err(|| "Could not create path")?;
        }

        let dirs = path.read_dir().chain_err(|| "Could not read files of path")?;
        let (files, dirs): (Vec<DirEntry>, Vec<DirEntry>) = dirs.filter_map(|x| x.ok()).partition(is_file);
        
        let files: Vec<String> = files.iter().filter_map(|d| d.file_name().into_string().ok()).collect();
        let dirs: Vec<String> = dirs.iter().filter_map(|d| d.file_name().into_string().ok()).collect();

        let c = Context::new()
            .insert("files", files)
            .insert("dirs", dirs)
            .insert("username", user.username)
            .insert("path", path_comps);

        println!("{:?}", c);

        Ok(c)
}

#[get("/upload/<file..>")]
fn get<'r>(user: auth::Auth, file: PathBuf) -> Result<FileWrapper<'r>> {

    let path = Path::new(&format!("upload/{}", user.username)).join(file.clone());
    if path.is_file() {
        let body = NamedFile::open(path).chain_err(|| "Could not open path")?;
        Ok(FileWrapper::Response(Response::build()
            .streamed_body(body)
            .finalize()))
    } else {
        let c = get_context(user, file)?;
        Ok(FileWrapper::Template(Template::render("files", &c.inner())))
    }
}

pub fn fuel(r: Rocket) -> Rocket {
    r.mount("/", routes![get, get_root, post])
}