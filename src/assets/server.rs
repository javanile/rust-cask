use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    update_frequency_sec: u32,
    num_threads: u32,
    data_sources: Vec<String>,
}

impl Server {
    pub fn auto_clone() {

        let f = std::fs::File::open("config.yml").expect("Could not open file.");
        //let mut scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
        let mut scrape_config: Server = Server {
            update_frequency_sec: 1,
            num_threads: 1,
            data_sources: vec!["Ciao".to_string()],
        };

        scrape_config.num_threads = 2;
        scrape_config
            .data_sources
            .push("www.nytimes.com".to_string());
        scrape_config
            .data_sources
            .push("news.yahoo.com".to_string());

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("new_config.yml")
            .expect("Couldn't open file");
        serde_yaml::to_writer(f, &scrape_config).unwrap();







    }
}