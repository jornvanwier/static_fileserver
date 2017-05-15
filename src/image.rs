use std::path::{PathBuf, Path};
use std::io::{self,Error, ErrorKind};
use std::fs::*;
use std::ops::Add;
use rocket::data::{Data, DataStream};
use rocket::http::{Status, ContentType};
use rocket::response::Failure;
use multipart::server::{Multipart, MultipartFile};
use multipart::server::save::SaveResult;
use multipart::server::save::EntriesSaveResult;
use multipart::server::SaveResult::*;
use site_management::*;


// Todo: change result to redirect to page with all photos, and info on any failures
pub fn upload(path: &PathBuf, data: Data, content_type: ContentType) -> Result<Vec<PathBuf>, Failure> {
    let content_type = content_type.params.unwrap().string;
    if let Some(index) = content_type.find("boundary=") {
        let boundary = &content_type[(index + "boundary=".len())..];

        let mut mp = Multipart::with_body(data.open(), boundary);

        let mut file_vec = vec![];
        
        if let Some(path) = convert_path(path) {
            // TODO: Keep file extensions (foreach and match all files, upload each file individually)
            mp.foreach_entry(|mut field| {
                // Who don't want to keep appending to the same file, do we?
                let mut path = path.clone();
                if let Some(file) = field.data.as_file() {
                    path.push(generate_filename(&file.filename));
                    let result = file.save().with_path(&path);
                    match result {
                        Full(file) => {
                            println!("file {:?}", file);
                            file_vec.push(file.path);
                        },
                        Partial(file, e) => {
                            // Partial most likely means failure
                            println!("partial {:?} {:?}", file, e);
                            remove_file(file.path);
                        },
                        Error(_) => {}
                        }
                    }
                });
            };

            return Ok(file_vec);
    }
    Err(Failure(Status::InternalServerError))
}

fn generate_filename(original_filename: &Option<String>) -> String {
    let mut name = rand_str(20);
    if let &Some(ref original_name) = original_filename {
        name = name.add(&get_extension(original_name).unwrap_or("".to_string()));
    }

    name
}

fn get_extension<'a>(name: &String) -> Option<String> {
    let split: Vec<&str> = name.split(".").collect();
    
    if split.len() == 0 {
        return None
    }

    // Name starts or ends with .
    if split[0].len() == name.len() - 1 {
        return None
    }

    split.last().map(|s| [".", s].concat().to_string())
}

fn convert_path(windows_path: &PathBuf) -> Option<PathBuf> {
    if let Some(path) = windows_path.to_str() {
        return Some(PathBuf::from(path.to_string().replace("\\", "/")))
    }
    None
}