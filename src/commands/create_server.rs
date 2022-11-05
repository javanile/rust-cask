
use crate::assets::server::Server;

pub fn run(server_name: String) {
    if Server::exists(server_name.clone()) {
        println!("Update {}", server_name);
    } else {
        println!("Creating {}", server_name);
        Server::create_server_file(server_name.clone())
    }

    Server::auto_clone();
}
