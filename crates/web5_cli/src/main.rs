mod dids;
mod pds;
mod test;
mod utils;
mod vcs;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "web5",
    about = "A decentralized web platform that puts you in control of your data and identity."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
#[allow(clippy::large_enum_variant)]
enum Commands {
    Did {
        #[command(subcommand)]
        did_command: dids::Commands,
    },
    Vc {
        #[command(subcommand)]
        vc_command: vcs::Commands,
    },
    Pd {
        #[command(subcommand)]
        pd_command: pds::Commands,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Did { did_command } => did_command.command().await,
        Commands::Vc { vc_command } => vc_command.command().await,
        Commands::Pd { pd_command } => pd_command.command().await,
    }
}
