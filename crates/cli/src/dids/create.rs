use clap::Subcommand;
use std::sync::Arc;
use web5::{
    crypto::dsa::ed25519::{Ed25519Generator, Ed25519Signer},
    dids::methods::{did_dht::DidDht, did_jwk::DidJwk},
};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Jwk,
    Web { domain: String },
    Dht,
}

impl Commands {
    pub fn command(&self) {
        match self {
            Commands::Jwk => {
                let mut private_jwk = Ed25519Generator::generate();
                private_jwk.d = None;
                let public_jwk = private_jwk.clone();

                let did_jwk = DidJwk::from_public_jwk(public_jwk).unwrap();
                println!(
                    "{}",
                    serde_json::to_string_pretty(&did_jwk.document).unwrap()
                );
            }
            Commands::Web { domain: _ } => {
                println!("🚧 not currently supported 🚧");
            }
            Commands::Dht => {
                let private_jwk = Ed25519Generator::generate();
                let signer = Ed25519Signer::new(private_jwk.clone());
                let mut identity_key = private_jwk.clone();
                identity_key.d = None;

                let did_dht = DidDht::from_identity_key(identity_key).unwrap();
                did_dht.publish(Arc::new(signer)).unwrap();

                println!(
                    "{}",
                    serde_json::to_string_pretty(&did_dht.document).unwrap()
                );
            }
        }
    }
}
