use clap::Subcommand;
use std::sync::Arc;
use web5::{
    crypto::dsa::ed25519::{Ed25519Generator, Ed25519Signer},
    dids::{
        methods::{did_dht::DidDht, did_jwk::DidJwk},
        portable_did::PortableDid,
    },
};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Jwk {
        #[arg(short, long)]
        no_indent: bool,
        #[arg(short, long)]
        json_escape: bool,
    },
    Web {
        domain: String,
    },
    Dht,
}

impl Commands {
    pub fn command(&self) {
        match self {
            Commands::Jwk {
                no_indent,
                json_escape,
            } => {
                let private_jwk = Ed25519Generator::generate();
                let mut public_jwk = private_jwk.clone();
                public_jwk.d = None;

                let did_jwk = DidJwk::from_public_jwk(public_jwk).unwrap();

                let portable_did = PortableDid {
                    did_uri: did_jwk.did.uri,
                    document: did_jwk.document,
                    private_jwks: vec![private_jwk],
                };

                let mut output_str = match no_indent {
                    true => serde_json::to_string(&portable_did).unwrap(),
                    false => serde_json::to_string_pretty(&portable_did).unwrap(),
                };

                if *json_escape {
                    output_str = output_str.replace("\"", "\\\"");
                }

                println!("{}", output_str);
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
