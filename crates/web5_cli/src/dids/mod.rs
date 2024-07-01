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
    pub fn command(&self) {
        match self {
            Commands::Resolve { uri } => {
                let resolution_result = ResolutionResult::new(uri);
                match &resolution_result.resolution_metadata.error {
                    Some(e) => println!("{:?} {}", e, e),
                    None => match &resolution_result.document {
                        None => println!(
                            "{:?} {}",
                            ResolutionMetadataError::InternalError,
                            ResolutionMetadataError::InternalError
                        ),
                        Some(document) => match serde_json::to_string_pretty(&document) {
                            Ok(s) => println!("{}", s),
                            Err(_) => println!(
                                "{:?} {}",
                                ResolutionMetadataError::InternalError,
                                ResolutionMetadataError::InternalError
                            ),
                        },
                    },
                }
            }
            Commands::Create { did_create_command } => did_create_command.command(),
        }
    }
}
