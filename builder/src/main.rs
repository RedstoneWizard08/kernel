pub mod commands;
pub mod config;
pub mod data;
pub mod docker;
pub mod logger;

use clap::Parser;
use commands::build::run_build;
use data::{Cli, Commands};

#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { clean, verbose } => {
            run_build(clean.clone(), verbose.clone()).await;
        }

        _ => {}
    }
}
