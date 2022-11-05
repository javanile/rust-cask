
extern crate dirs;

use std::fs;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    name: String,
    repo: String,
    //workdir: String,
    //host: String,
    //port: String,
    //private_key: String
}

impl Server {

    pub fn exists(server_name: String) -> bool {
        return false;
    }

    pub fn create_server_file(server_name: String) {
        let home_dir : String = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
        let server_dir : String = format!("{}/{}", home_dir, ".cask");
        let server_file : String = format!("{}/{}.{}", server_dir, server_name, "yml");

        println!("FILE: {}", server_file);

        fs::create_dir_all(server_dir).unwrap();

        let mut server_config: Server = Server {
            name: "1".to_string(),
            repo: "1".to_string(),
            //data_sources: vec!["Ciao".to_string()],
        };

        //server_config.num_threads = 2;
        //server_config.data_sources.push("www.nytimes.com".to_string());
        //server_config.data_sources.push("news.yahoo.com".to_string());

        let file_writer = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(server_file)
            .expect("Couldn't open file");

        serde_yaml::to_writer(file_writer, &server_config).unwrap();
    }

    pub fn auto_clone() {

        let f = std::fs::File::open("config.yml").expect("Could not open file.");
        //let mut scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
        let mut scrape_config: Server = Server {
            name: "1".to_string(),
            repo: "1".to_string(),
            //update_frequency_sec: 1,
            //num_threads: 1,
            //data_sources: vec!["Ciao".to_string()],
        };

        scrape_config.name = "2".to_string();
        //scrape_config
//            .data_sources
//            .push("www.nytimes.com".to_string());
//        scrape_config
//            .data_sources
 //           .push("news.yahoo.com".to_string());

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("new_config.yml")
            .expect("Couldn't open file");
        serde_yaml::to_writer(f, &scrape_config).unwrap();
    }
}
