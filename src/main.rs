use std::{env, fs, path::PathBuf};
use threadpool::Builder as threadpool_Builder;

fn scan_dir(current_dir: &PathBuf, files: &mut Vec<PathBuf>) {
    println!("Reading files in {}", &current_dir.display());

    let mut args = env::args();
    let dir_name = args.nth(1).unwrap_or("node_modules".to_string());

    for entry in fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.ends_with(&dir_name) {
            files.push(path);
        } else {
            let meta = fs::metadata(&path).unwrap();

            if meta.is_dir() {
                scan_dir(&entry.path(), files);
            }
        }
    }
}

fn main() {
    let cpus = num_cpus::get();
    let thread_pool_size = cpus / 2;

    let pool = threadpool_Builder::new().num_threads(thread_pool_size);

    let current_dir = env::current_dir().unwrap();

    let mut dirs: Vec<PathBuf> = Vec::new();

    scan_dir(&current_dir, &mut dirs);

    for entry in dirs {
        fs::remove_dir_all(entry).expect("Failed to remove dir");
    }
}
