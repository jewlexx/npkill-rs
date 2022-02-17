use spinners_rs::{Spinner, Spinners};
use std::{env, path::PathBuf, str::FromStr};
use threadpool::Builder as threadpool_Builder;

mod utils;
use utils::*;

fn main() {
    let args = args::parse_args();

    let current_dir = PathBuf::from_str(&args.dir).unwrap();

    let mut dirs: Vec<PathBuf> = Vec::new();

    let sp = Spinner::new(Spinners::Aesthetic, "Scanning directories...".into());
    dir::scan_dir(&current_dir, &mut dirs);
    sp.stop();

    let cpus = num_cpus::get();
    let thread_pool_size = cpus / 2;

    let pool = threadpool_Builder::new()
        .num_threads(thread_pool_size)
        .build();

    for entry in dirs {
        pool.execute(move || {
            println!("Deleting dir {}", &entry.as_path().display());
            // fs::remove_dir_all(entry).expect("Failed to remove dir");
        });
    }

    pool.join();
}
