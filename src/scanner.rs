
use walkdir::{DirEntry, WalkDir, Error};


pub fn walk(root_dir: String) -> (Vec<DirEntry>,  Vec<Error>) {
    let mut entries: Vec<DirEntry> = vec![];
    let mut errors: Vec<Error> = vec![];
    let walk = WalkDir::new(root_dir);

    for entry in walk.into_iter().filter_entry(skip) {
        match entry {
            Ok(s) =>  entries.push(s),
            Err(err) => errors.push(err)
        }
    }
    (entries, errors)
}

pub fn to_report(entries: Vec<DirEntry>) {
    let mut files: Vec<String> = vec![];
    let mut dirs: Vec<String> = vec![];

    for dir in entries.into_iter() {
        let path = dir.path();
        if path.is_dir() {
            dirs.push(String::from(path.to_str().unwrap()));
        }

        if path.is_file() {
            files.push(String::from(path.to_str().unwrap()))
        }
    }

    println!("files");
    for path in files.into_iter() {
        println!("\t - {}", path)
    }

    println!("dirs");
    for path in dirs.into_iter() {
        println!("\t - {}", path)
    }
}

pub fn skip(entry: &DirEntry) -> bool {
    !entry
        .file_name()
        .to_str()
        .map(|name| entry.path().is_dir() && name == "target" || name == ".git")
        .unwrap_or(false)
}
