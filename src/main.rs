use std::{env, fs, path::PathBuf};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let mut files: Vec<PathBuf> = vec![];

    println!("Reading files in {}", &current_dir.display());

    for entry in fs::read_dir(&current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.ends_with("node_modules") {
            files.push(path);
        } else {
            let meta = fs::metadata(&path).unwrap();
        }
    }
}
