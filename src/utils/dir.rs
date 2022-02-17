use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn scan_dir(current_dir: &Path, files: &mut Vec<PathBuf>) {
    let mut args = env::args();
    let dir_name = args.nth(1).unwrap_or_else(|| "node_modules".to_string());

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
