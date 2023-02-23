use std::{fs::remove_file, path::Path};

use crate::logger::Logger;
use walkdir::WalkDir;

pub const CLEAN_LOGGER: Logger = Logger::new("CLEAN");

// This is so we can have the recursive functionality.
pub fn remove_recursive(target_dir: &str, verbose: bool) {
    if verbose {
        CLEAN_LOGGER.debug(format!("Removing directory: {}", target_dir));
    }

    for entry in WalkDir::new(target_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        remove_file(path).unwrap();
    }
}

pub fn run_clean(verbose: bool) {
    remove_recursive("target", verbose);

    if Path::new("kernel8.img").exists() && Path::new("kernel8.img").is_file() {
        if verbose {
            CLEAN_LOGGER.debug("Removing file: kernel8.img".to_string());
        }

        remove_file("kernel8.img").unwrap();
    }
}
