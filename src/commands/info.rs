
use std::fs;
use crate::assets;

pub fn run() {
    let version: String = assets::local::version();
    let local_dir : String = assets::local::dir();

    println!("Cask running: v{}", version);
    println!("Local directory: {}", local_dir);

    if assets::local::default_exists() {

    } else {

    }

    println!("Local known servers:");
    for file in fs::read_dir(local_dir).unwrap() {
        println!(" - {}", file.unwrap().path().display());
    }
}
