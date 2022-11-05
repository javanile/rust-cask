use std::fs::read;
use crate::assets::server::get_server_file;
use std::process::{Command, Stdio};
use run_script::ScriptOptions;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn run(server_name: String) {
    /*
    if Server::exists(server_name.clone()) {
        println!("Update {}", server_name);
    } else {
        println!("Creating {}", server_name);
        Server::create_server_file(server_name.clone())
    }

    Server::auto_clone();
    */

    let server_file: String = get_server_file(server_name);

    use std::process::{Command, Stdio};

    let output = Command::new("nano")
        .arg("Hello, world!")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");
}
