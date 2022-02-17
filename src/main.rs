use spinners_rs::{Spinner, Spinners};
use std::{
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

mod utils;
use utils::*;

fn main() {
    let args = args::parse_args();

    let current_dir = PathBuf::from_str(&args.dir).unwrap();

    let shared_dirs = Arc::new(Mutex::new(Vec::<PathBuf>::new()));

    let sp = Spinner::new(Spinners::Aesthetic, "Scanning directories...".into());
    let cloned_dirs = Arc::clone(&shared_dirs);
    thread::spawn(move || {
        dir::scan_dir(&current_dir, cloned_dirs);
    });
    sp.stop();

    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    let new_vec: Vec<PathBuf> = shared_dirs.lock().unwrap().clone();
    for entry in new_vec {
        let thread = thread::spawn(move || {
            println!("Deleting dir {}", &entry.as_path().display());
            // fs::remove_dir_all(entry).expect("Failed to remove dir");
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().expect("Failed to join thread");
    }
}
