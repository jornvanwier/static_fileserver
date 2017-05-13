use rocket_contrib::Template;
use std::fs::DirEntry;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct DirectoryPage<'a> {
    pub title: &'a str,
    pub entries: &'a Vec<DirectoryEntry>,
}

#[derive(Debug, Serialize)]
pub struct DirectoryEntry {
    pub name: String,
    pub path: PathBuf,
}

pub fn get_file_emoji(file: &DirEntry) -> String {
    match file.path().is_dir() {
        true => "📁".to_string(),
        false => "📄".to_string()
    }
}

pub fn create_dir_view(path: &PathBuf) -> Template {
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
        title: &format!("/{}/", &path.to_string_lossy().into_owned()),
        entries: &result,
    };

    Template::render("directory", &context)
}