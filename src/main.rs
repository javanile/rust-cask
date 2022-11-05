
mod assets;
mod commands;

//use std::ffi::OsStr;
//use std::ffi::OsString;
//use std::path::PathBuf;
//use serde::{Deserialize, Serialize};
//use serde_yaml::{self};

use clap::{/*Args,*/ Parser, Subcommand, /*ValueEnum*/};

//use crate::assets::server::Server;
use crate::commands::{create_server, info};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
   /// Get local information
   #[command(name = "info")]
   Info,

   /// Create new server
   #[command(name = "create:server")]
   CreateServer {
      server_name: String,
   }
}

fn main() {
   let args = Cli::parse();

   match args.command {
      Commands::Info {} => {
         info::run()
      }
      Commands::CreateServer { server_name } => {
         create_server::run(server_name)
      }
   }
}
