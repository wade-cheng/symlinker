use std::{collections::HashSet, fs};
use std::os::unix::fs as unix_fs;
use walkdir::{DirEntry, WalkDir};

use crate::constants;

/// check that the cache directory exists, and if it does not, create it.
fn touch_cache_dir() {
    fs::create_dir(constants::CACHE_NAME).ok();
}

/// returns whether an entry should be symlinked.
/// the symlinker config TOML, hidden files, and directories are not to be symlinked
fn should_not_symlink(entry: &DirEntry) -> bool {
    // println!("entry filename: {}", entry.file_name().to_str().unwrap());

    let file_name = entry.file_name().to_str().unwrap();
    if file_name.starts_with('.') || file_name == constants::CONFIG_NAME {
        return true;
    }

    if entry.metadata().unwrap().is_dir() {
        return true;
    }

    // TODO STOPGAP WHAT IS GOING ON WHY DOES .CACHE NOT GET SKIPPED THOUGH HIDDEN ohhh https://docs.rs/walkdir/latest/walkdir/index.html
    if entry.path().to_str().unwrap().contains(".cache") {
        return true;
    }

    false
}

/// this is now unused
fn _get_cache_filenames() -> HashSet<String> {
    let mut names = HashSet::new();

    for entry in WalkDir::new(constants::CONFIG_NAME) {
        names.insert(entry
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .to_string());
    }

    names
}

pub fn symlink_all() {
    touch_cache_dir();

    // silently skip directories that the owner of the running process does not have permission to access
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if should_not_symlink(&entry) {
            continue;
        }

        if entry.metadata().unwrap().is_symlink() {
            continue;
        }

        println!("processing unsymlinked file at {}", entry.path().display());

        // move the file to cache and make symlink from its last position to the cache position
        let file_name = entry.file_name().to_str().unwrap();
        let file_path = entry.path();

        println!("  renaming file {} to cache", file_path.display());
        fs::rename(
            file_path,
            format!("{}/{}", constants::CACHE_NAME, file_name),
        )
        .unwrap();
    
        println!("  creating symlink from {} to cache", file_path.display());
        unix_fs::symlink(
            format!("../{}/{}", constants::CACHE_NAME, file_name),
            file_path.to_str().unwrap(),
        )
        .unwrap();
    }

    println!("finished.");
}
