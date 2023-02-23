#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate rustc_demangle;

pub mod commands;
pub mod config;
pub mod data;
pub mod demangler;
pub mod docker;
pub mod logger;
pub mod util;

use clap::Parser;
use commands::{build::run_build, clean::run_clean};
use data::{Cli, Commands};

#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { clean, verbose } => {
            run_build(clean.clone(), verbose.clone()).await;
        }

        Commands::Clean { verbose } => {
            run_clean(verbose.clone());
        }

        _ => {}
    }
}
