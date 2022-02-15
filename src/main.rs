use std::{collections::HashMap, env, fs, path::PathBuf, str::FromStr};
use threadpool::Builder as threadpool_Builder;

fn scan_dir(current_dir: &PathBuf, files: &mut Vec<PathBuf>) {
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

    println!("Searching Dirs...");
    scan_dir(&current_dir, &mut dirs);

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
