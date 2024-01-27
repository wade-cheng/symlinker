use std::fs;

use crate::constants;

pub fn create_archive(name: &String) {
    fs::create_dir(name).expect("A directory with this name already exists");

    fs::create_dir(format!("{}/{}", name, constants::CACHE_NAME)).unwrap();

    fs::create_dir(format!("{}/media", name)).unwrap();

    fs::write(
        format!("{}/{}", name, constants::CONFIG_NAME),
        constants::CONFIG_PRESET,
    )
    .expect("Unable to write file");

    println!("Successfully created new archive `{}`", name)
}
