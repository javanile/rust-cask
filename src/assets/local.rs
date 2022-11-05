
extern crate dirs;

//use std::fs;

pub fn dir() -> String {
    let home_dir : String = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
    let local_dir : String = format!("{}/{}", home_dir, ".cask");

    return local_dir;
}

pub fn version() -> String {
    let version: String = env!("CARGO_PKG_VERSION").to_string();

    return version
}
