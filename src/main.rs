use std::{env, fs, path::PathBuf};

fn main() {
    let current_dir = env::current_dir().unwrap();
    let mut args = env::args();
    let dir_name = args.nth(1).unwrap_or("node_modules".to_string());

    let mut files: Vec<PathBuf> = Vec::new();

    println!("Reading files in {}", &current_dir.display());

    for entry in fs::read_dir(&current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.ends_with(&dir_name) {
            files.push(path);
        } else {
            let meta = fs::metadata(&path).unwrap();
        }
    }
}
