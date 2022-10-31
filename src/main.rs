mod assets;

use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::assets::server::Server;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
   /// Create new server
   #[command(name = "create:server")]
   CreateServer {
      /// The remote to clone
      server_name: String,
   }
}

fn main() {
   let args = Cli::parse();

   match args.command {
      Commands::CreateServer { server_name } => {
         if Server::exists(server_name.clone()) {
            println!("Update {}", server_name);
         } else {
            println!("Creating {}", server_name);
         }

         Server::auto_clone();
      }
   }
}
