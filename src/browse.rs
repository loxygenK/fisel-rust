use std::{fs, io::Error};

#[derive(Debug)]
pub struct BrowsedDirectory {
    path: String,
    files: Vec<String>,
    dirs: Vec<String>
}

pub fn browse(path: &str) -> Result<BrowsedDirectory, Error> {
    let dir = fs::read_dir(path)?.filter_map(|p| p.ok());

    let mut files: Vec<String> = vec![];
    let mut dirs: Vec<String> = vec![];

    for child in dir {
        if let Ok(name) = child.file_name().into_string() {
            if child.path().is_file() {
                files.push(name);
            } else if child.path().is_dir() {
                dirs.push(name);
            }
        }
    }

    Ok(
        BrowsedDirectory {
            path: path.to_string(), files, dirs
        }
    )
}
