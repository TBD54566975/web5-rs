mod dids;
mod pds;
mod test;
mod utils;
mod vcs;

use std::fs::File;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "web5",
    about = "A decentralized web platform that puts you in control of your data and identity."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// A file to output command output to.
    #[arg(long, global = true)]
    output: Option<String>,
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

    if let Some(path) = cli.output {
        let file = File::create(path).unwrap();
        command(cli.command, file).await;
    } else {
        command(cli.command, std::io::stdout()).await;
    }
}

async fn command(command: Commands, sink: impl std::io::Write) {
    match command {
        Commands::Did { did_command } => did_command.command(sink).await,
        Commands::Vc { vc_command } => vc_command.command(sink).await,
        Commands::Pd { pd_command } => pd_command.command(sink).await,
    }
}
