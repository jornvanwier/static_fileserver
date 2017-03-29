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
use std::fs::DirEntry;


#[derive(Debug, Serialize)]
struct DirectoryPage<'a> {
    title: &'a str,
    entries: &'a Vec<DirectoryEntry>,
}

#[derive(Debug, Serialize)]
struct DirectoryEntry {
    name: String,
    path: PathBuf,
}

fn get_file_emoji(file: &DirEntry) -> String {
    match file.path().is_dir() {
        true => "📁".to_string(),
        false => "📄".to_string()
    }
}

fn create_dir_view(path: &PathBuf) -> Template {
    let mut result = Vec::new();

    if let Ok(files) = path.read_dir() {
        for file in files {
            if let Ok(file) = file {
                result.push(DirectoryEntry {
                                name: get_file_emoji(&file) + " " + &(file.file_name()
                                    .as_os_str()
                                    .to_string_lossy()
                                    .into_owned()),
                                path: file.path().to_path_buf()
                            });
            }
        }
    }

    let context = DirectoryPage {
        title: path.to_str().unwrap(),
        entries: &result,
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
    create_dir_view(&PathBuf::from("files"))
}

// Seperate css from other files
#[get("/static/<file..>")]
fn get_static(file: PathBuf) -> NamedFile {
    NamedFile::open(Path::new("static/").join(file)).unwrap()
}

fn main() {
    rocket::ignite().mount("/", routes![get_files, get_dir, get_root_dir, get_static]).launch();
}
