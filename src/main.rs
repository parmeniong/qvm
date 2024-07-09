mod install;
mod http;

use install::install;
use semver::Version;
use clap::{Parser, Subcommand};

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
            let version = Version::parse(&version).unwrap();
            install(version);
        }
    }
}