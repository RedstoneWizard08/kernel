pub mod data;
pub mod commands;

use data::{Commands, Cli};
use clap::Parser;

#[tokio::main]
pub fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { clean, verbose } => {
            println!("Build called!");
        }

        _ => {},
    }
}
