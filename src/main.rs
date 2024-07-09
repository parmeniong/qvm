mod install;
mod http;
mod util;

use install::install;
use semver::Version;
use util::{error, ErrorType};
use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Subcommand)]
enum Command {
    /// Install a version of Quasar
    Install {
        /// The version of Quasar to install
        version: String
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Install { version } => {
            let parsed_version: Version;
            if let Ok(result) = semver::Version::parse(version) {
                parsed_version = result;
            } else {
                error(ErrorType::VersionError, format!("Invalid version: {}", version));
                exit(1);
            }

            install(parsed_version);
        }
    }
}