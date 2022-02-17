use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

pub fn scan_dir(current_dir: &Path, files: Arc<Mutex<Vec<PathBuf>>>) {
    let mut args = env::args();
    let dir_name = args.nth(1).unwrap_or_else(|| "node_modules".to_string());
    let mut mut_files = files.lock().unwrap();

    for entry in fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.ends_with(&dir_name) {
            mut_files.push(path);
        } else {
            let meta = fs::metadata(&path).unwrap();

            if meta.is_dir() {
                let cloned_files = Arc::clone(&files);
                thread::spawn(move || {
                    scan_dir(&path, cloned_files);
                });
            }
        }
    }
}
