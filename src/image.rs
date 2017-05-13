use std::path::{PathBuf, Path};
use std::io::{self,Error, ErrorKind};
use std::fs::*;
use rocket::data::{Data, DataStream};
use rocket::http::{Status, ContentType};
use rocket::response::Failure;
use multipart::server::Multipart;
use multipart::server::save::SaveResult;
use multipart::server::save::EntriesSaveResult;
use multipart::server::SaveResult::*;

pub fn upload(path: &PathBuf, data: Data, content_type: ContentType) -> Result<PathBuf, Failure> {
    let content_type = content_type.params.unwrap().string;
    if let Some(index) = content_type.find("boundary=") {
        let boundary = &content_type[(index + "boundary=".len())..];

        let mut mp = Multipart::with_body(data.open(), boundary);
        
        if let Some(path) = convert_path(path) {
            let result = mp.save().with_dir(path);

            return match result {
                Full(entries) => {
                    println!("scuessfol uplaad");
                    if !entries.is_empty() {
                        if let Some(files) = entries.files.values().next() {
                            if files.len() > 0 {
                                let path = &files[0].path;
                                return Ok(path.to_path_buf())
                            }
                        }
                    }
                    Err(Failure(Status::InternalServerError))
                },
                Partial(partial_entries, e) => {
                    println!("partial uplaod");
                    // Cleanup partial and return any result (likely none)
                    println!("{:#?}, {:#?}", partial_entries.entries, e);
                    if let Some(partial) = partial_entries.partial_file {
                        if let Some(dest) = partial.dest {
                            remove_file(dest.path);
                        }
                    }
                    Err(Failure(Status::InternalServerError))
                },
                Error(_) => Err(Failure(Status::InternalServerError))
            }
            
        }
    }
    Err(Failure(Status::InternalServerError))
}

fn convert_path(windows_path: &PathBuf) -> Option<PathBuf> {
    if let Some(path) = windows_path.to_str() {
        return Some(PathBuf::from(path.to_string().replace("\\", "/")))
    }
    None
}