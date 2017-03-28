#![feature(plugin)]
#![plugin(rocket_codegen)]

mod path_dir;

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use path_dir::PathDir;
use std::ffi::{OsStr, OsString};


#[derive(Debug, Serialize)]
struct DirectoryPage<'a> {
    title: &'a str,
    entries: &'a Vec<DirectoryEntry>,
}

#[derive(Debug, Serialize)]
struct DirectoryEntry {
    name: String,
    path: String,
}

// TODO remove directory component from path
fn create_dir_view(path: &PathBuf) -> Template {
    println!("Constructing dir view");
    let mut files = Vec::new();

    match path.read_dir() {
        Ok(x) => {
            for file in x {
                files.push(match file {
                               Ok(x) => {
                                   match x.path().strip_prefix("files/") {
                                       Ok(full_path) => {
                                           if let Some(path_os_str) = x.path().file_name() {
                                               DirectoryEntry {
                                                   name: path_os_str.to_string_lossy().into_owned(),
                                                   path: match full_path.to_str() {
                                                       Some(path_str) if full_path.is_dir() => {
                                                           path_str.to_string() + "/"
                                                       }
                                                       Some(path_str) => path_str.to_string(),
                                                       None => continue,
                                                   },
                                               }
                                           } else {
                                               continue;
                                           }
                                       }
                                       Err(_) => continue,
                                   }
                               }
                               Err(_) => continue,
                           })
            }
        }
        Err(e) => println!("{}", e),
    }

    let context = DirectoryPage {
        title: path.to_str().unwrap(),
        entries: &files,
    };

    Template::render("directory", &context)
}

#[get("/files/<file..>", rank = 1)]
fn get_files(file: PathBuf) -> NamedFile {
    NamedFile::open(Path::new("files/").join(file)).unwrap()
}

#[get("/files/<dir..>", rank = 0)]
fn get_dir(dir: PathDir) -> Template {
    create_dir_view(&dir)
}

// Show directory contents for root
#[get("/files")]
fn get_root_dir() -> Template {
    create_dir_view(&PathBuf::from("files/"))
}

fn main() {
    rocket::ignite().mount("/", routes![get_files, get_dir, get_root_dir]).launch();
}
