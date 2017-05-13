#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate multipart;
extern crate site_management;

mod image;
mod directory;
mod path_dir;

use rocket_contrib::Template;
use rocket::response::{Failure, NamedFile};
use rocket::State;
use rocket::data::Data;
use rocket::http::{Status, ContentType};
use rocket::outcome::Outcome;
use std::path::{self, Path, PathBuf};
use path_dir::PathDir;
use std::env;

use site_management::*;
use site_management::user_login::*;

use directory::*;

#[post("/upload/<session_key>/<path..>", format = "multipart/form-data", data="<image>")]
fn upload_file(session_key: String, path: PathBuf, image: Data, content_type: ContentType, pool: State<ConnectionPool>) -> Result<String, Failure> {
    if let Outcome::Success(login) = UserLogin::from_key(session_key, &*pool) {

        if path.extension() != None || path.to_str() == None {
            return Err(Failure(Status::BadRequest))
        }

        let mut path = Path::new("files/").join(path);

        println!("'{}'", &login.user.username);
        path.push(&login.user.username);

        let result = image::upload(&path, image, content_type);

        Ok("ha".to_string())
    }
    else {
        Err(Failure(Status::Unauthorized))
    }
}

#[get("/files/<file..>", rank = 1)]
fn get_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("files/").join(file)).ok()
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
fn get_static(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    println!("Running from {:?}", env::current_dir().unwrap());
    println!("Environment is {}", env::var("ROCKET_ENV").unwrap_or("not set".to_string()));
    rocket::ignite().mount("/", routes![
            upload_file,
            get_files, 
            get_dir, 
            get_root_dir, 
            get_static])
        .manage(establish_connection_pool())
        .launch();
}
