use spinners_rs::{Spinner, Spinners};
use std::{
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};
use threadpool::Builder as threadpool_Builder;

mod utils;
use utils::*;

fn main() {
    let args = args::parse_args();

    let current_dir = PathBuf::from_str(&args.dir).unwrap();

    let shared_dirs = Arc::new(Mutex::new(Vec::<PathBuf>::new()));

    let cpus = num_cpus::get();
    let thread_pool_size = cpus / 2;

    let pool = threadpool_Builder::new()
        .num_threads(thread_pool_size)
        .build();

    let sp = Spinner::new(Spinners::Aesthetic, "Scanning directories...".into());
    dir::scan_dir(&current_dir, Arc::clone(&shared_dirs));
    sp.stop();

    pool.join();

    let pool = threadpool_Builder::new()
        .num_threads(thread_pool_size)
        .build();

    let new_vec: &'static Vec<PathBuf> = &shared_dirs.lock().unwrap().clone();
    for entry in new_vec {
        pool.execute(move || {
            println!("Deleting dir {}", &entry.as_path().display());
            // fs::remove_dir_all(entry).expect("Failed to remove dir");
        });
    }
}
