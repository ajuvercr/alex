
use rocket::response::{Response, NamedFile};
use rocket::{Route, Rocket};

use std::io::Cursor;
use std::ffi::OsString;
use std::io::{Write};
use std::fs::{File, create_dir_all, DirEntry};
use std::path::{Path, PathBuf};
use std::io;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
enum FileType {
    Folder, File, SysLink, None
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct OSFile {
    name: String,
    file_type: FileType,
}

impl OSFile {
    fn new(f: DirEntry) -> OSFile {
        let file_type = match f.file_type() {
            Ok(ft) => {
                if ft.is_file() {
                    FileType::File
                } else if ft.is_dir() {
                    FileType::Folder
                } else {
                    FileType::SysLink
                }
            },
            Err(_) => FileType::None
        };

        let name = match f.file_name().into_string() {
            Ok(s) => s,
            Err(e) => "ERROR, NO FUCKING FILE".to_string(),
        };

        OSFile {
            name, file_type
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Folder {
    name: String,
    children: Vec<OSFile>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Out {
    token: String,
    folder: Folder,
}

impl Out {
    fn new(name: PathBuf, children: Vec<OSFile>) -> Out {
        let token = "My Super Secret Token".to_string();
        let folder = Folder {
            name: name.to_str().unwrap_or("").to_string(), children
        };

        Out {
            folder, token
        }
    }
}


#[post("/upload/<path..>", data = "<data>")]
pub fn post(data: Vec<u8>, path: PathBuf) -> io::Result<String> {
    let mut path = Path::new("./upload").join(path);
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
pub fn get_root<'a>() -> Option<Response<'a>> {
    get(PathBuf::new())
}

#[get("/upload/<path..>")]
pub fn get<'a>(path: PathBuf) -> Option<Response<'a>> {
    let path = Path::new("./upload").join(path);

    if path.is_file() {
        println!("getting {:?}", path);
        match NamedFile::open(path) {
            Ok(body) => Some(Response::build()
                .streamed_body(body)
                .finalize()),
            Err(e) => {
                println!("Error {:?}", e);
                None
            }
        }
    }else{
        if let Ok(dirs) = path.read_dir() {
            let dirs: Vec<OSFile> = dirs
                .filter_map(|d| 
                    d.ok().map(|d| OSFile::new(d))).collect();
            let out = Out::new(path, dirs);
            Some(Response::build()
                .streamed_body(Cursor::new(serde_json::to_string(&out).unwrap()))
                .finalize())

        } else {
            Some(Response::build()
                .streamed_body(Cursor::new("you made a fucking mistake"))
                .finalize())
        }
    }
}

pub fn fuel(r: Rocket) -> Rocket {
    r.mount("/", routes![get, get_root, post])
}