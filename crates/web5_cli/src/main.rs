mod dids;
mod doctor;
mod test;
mod utils;
mod vcs;

use clap::{ Parser, Subcommand, ValueEnum };
use doctor::{ print_health_check_results, run_health_checks };

#[derive(Parser, Debug)]
#[command(
    name = "web5",
    about = "A decentralized web platform that puts you in control of your data and identity."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, ValueEnum)]
enum CheckType {
    CliVersion,
    Dependencies,
    EnvVars,
    Connectivity,
    BasicFunctionality,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Did {
        #[command(subcommand)]
        did_command: dids::Commands,
    },
    Vc {
        #[command(subcommand)]
        vc_command: vcs::Commands,
    },
    Doctor {
        #[arg(long, value_enum)]
        check: Option<CheckType>, // Optional argument for individual checks
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Did { did_command } => did_command.command().await,
        Commands::Vc { vc_command } => vc_command.command().await,
        Commands::Doctor { check } => {
            let state = run_health_checks(check).await;
            print_health_check_results(&state);
        }
    }
}
