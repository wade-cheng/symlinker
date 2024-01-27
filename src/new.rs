use std::fs;

use crate::constants;

/// Creates a new archive given a name.
pub fn create_archive(name: &String) {
    // create the archive directory
    fs::create_dir(name).expect("A directory with this name already exists");

    // create cache directory
    fs::create_dir(format!("{}/{}", name, constants::CACHE_NAME)).unwrap();

    // create template directory for placing new media
    fs::create_dir(format!("{}/media", name)).unwrap();

    // create config TOML with preset "magic number"
    fs::write(
        format!("{}/{}", name, constants::CONFIG_NAME),
        constants::CONFIG_PRESET,
    )
    .expect("Unable to write file");

    // finish function
    println!("Successfully created new archive `{}`", name)
}
