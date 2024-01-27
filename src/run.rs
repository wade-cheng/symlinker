use std::fs;
use std::os::unix::fs as unix_fs;
use walkdir::{DirEntry, WalkDir};

use crate::constants;

/// Check that the cache directory exists, and if it does not, create it.
fn touch_cache_dir() {
    fs::create_dir(constants::CACHE_NAME).ok();
}

/// Returns whether an entry should be symlinked.
///
/// Symlinker metadata files, directories, and symlinks should not be symlinked.
fn should_not_symlink(entry: &DirEntry) -> bool {
    let file_name: &str = entry.file_name().to_str().unwrap();

    // disallow symlinking the config TOML
    if file_name == constants::CONFIG_NAME {
        return true;
    }

    // disallow symlinking the cache
    if entry
        .path()
        .to_str()
        .unwrap()
        .contains(constants::CACHE_NAME)
    {
        return true;
    }

    // disallow symlinking directories
    if entry.metadata().unwrap().is_dir() {
        return true;
    }

    // disallow symlinking symlinks
    if entry.metadata().unwrap().is_symlink() {
        return true;
    }

    false
}

/// Symlink all files in the current archive that should be symlinked.
pub fn symlink_all() {
    touch_cache_dir();

    // silently skip directories that the owner of the running process does not have permission to access
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        // ignore all files that should not be symlinked
        if should_not_symlink(&entry) {
            continue;
        }

        println!("processing unsymlinked file at {}", entry.path().display());

        // the remainder of this for loop moves the file to the cache and makes a symlink from its last position to its cache position
        // moving is probably as efficient of a file operation as we can get
        let file_name = entry.file_name().to_str().unwrap();
        let file_path = entry.path();

        println!("  moving file {} to cache", file_path.display());
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
