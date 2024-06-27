use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Create { credential_subject_id: String },
    Sign { vc: String, portable_did: String },
    Verify { vc_jwt: String },
}

impl Commands {
    pub fn command(&self) {
        match self {
            Commands::Create {
                credential_subject_id,
            } => {
                println!(
                    "Creating VC for credential subject ID: {}",
                    credential_subject_id
                );
                // Implement VC creation logic here
            }
            Commands::Sign { vc, portable_did } => {
                println!("Signing VC: {} with portable DID: {}", vc, portable_did);
                // Implement VC signing logic here
            }
            Commands::Verify { vc_jwt } => {
                println!("Verifying VC-JWT: {}", vc_jwt);
                // Implement VC-JWT verification logic here
            }
        }
    }
}
