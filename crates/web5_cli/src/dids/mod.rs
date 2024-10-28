mod create;

use clap::Subcommand;
use web5::dids::resolution::{
    resolution_metadata::ResolutionMetadataError, resolution_result::ResolutionResult,
};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Resolve {
        uri: String,
    },
    Create {
        #[command(subcommand)]
        did_create_command: create::Commands,
    },
}

impl Commands {
    pub async fn command(&self, mut sink: impl std::io::Write) {
        match self {
            Commands::Resolve { uri } => {
                let resolution_result = ResolutionResult::resolve(uri).await;
                match &resolution_result.resolution_metadata.error {
                    Some(e) => eprintln!("{:?} {}", e, e),
                    None => match &resolution_result.document {
                        None => eprintln!(
                            "{:?} {}",
                            ResolutionMetadataError::InternalError,
                            ResolutionMetadataError::InternalError
                        ),
                        Some(document) => match serde_json::to_string_pretty(&document) {
                            Ok(s) => writeln!(sink, "{}", s).unwrap(),
                            Err(_) => eprintln!(
                                "{:?} {}",
                                ResolutionMetadataError::InternalError,
                                ResolutionMetadataError::InternalError
                            ),
                        },
                    },
                }
            }
            Commands::Create { did_create_command } => did_create_command.command(sink).await,
        }
    }
}
