use spinners_rs::{Spinner, Spinners};
use std::{collections::HashMap, env, path::PathBuf, str::FromStr};
use threadpool::Builder as threadpool_Builder;

mod utils;
use utils::*;

fn main() {
    let mut args = HashMap::<String, String>::new();

    for (i, el) in env::args().enumerate() {
        if i == 0 {
            continue;
        }

        if el.starts_with("--") {
            if el.contains('=') {
                let mut el = el.splitn(2, '=');
                let key = el.next().unwrap();
                let value = el.next().unwrap();

                args.insert(key.to_string(), value.to_string());
                continue;
            }
            args.insert(
                el.to_string(),
                env::args().enumerate().nth(i + 1).unwrap().1,
            );
        }
    }

    let current_dir = PathBuf::from_str(
        args.get("--dir")
            .unwrap_or(&env::current_dir().unwrap().display().to_string()),
    )
    .unwrap();

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
