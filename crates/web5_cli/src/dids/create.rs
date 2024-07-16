use clap::Subcommand;
use std::sync::Arc;
use url::Url;
use web5::{
    crypto::dsa::ed25519::{Ed25519Generator, Ed25519Signer},
    dids::{
        methods::{did_dht::DidDht, did_jwk::DidJwk, did_web::DidWeb},
        portable_did::PortableDid,
    },
};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Jwk {
        #[arg(long)]
        no_indent: bool,
        #[arg(long)]
        json_escape: bool,
    },
    Web {
        domain: String,
        #[arg(long)]
        no_indent: bool,
        #[arg(long)]
        json_escape: bool,
    },
    Dht {
        #[arg(long)]
        no_publish: bool,
        #[arg(long)]
        no_indent: bool,
        #[arg(long)]
        json_escape: bool,
    },
}

fn print_portable_did(portable_did: PortableDid, no_indent: &bool, json_escape: &bool) {
    let mut output_str = match no_indent {
        true => serde_json::to_string(&portable_did).unwrap(),
        false => serde_json::to_string_pretty(&portable_did).unwrap(),
    };

    if *json_escape {
        output_str = output_str.replace('"', "\\\"");
    }

    println!("{}", output_str);
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

                print_portable_did(portable_did, no_indent, json_escape);
            }
            Commands::Web {
                domain,
                no_indent,
                json_escape,
            } => {
                let private_jwk = Ed25519Generator::generate();
                let mut public_jwk = private_jwk.clone();
                public_jwk.d = None;

                let valid_url = if domain.starts_with("http://") || domain.starts_with("https://") {
                    let url = Url::parse(domain).expect("Invalid URL");

                    // Ensure "http://" is only allowed for localhost or 127.0.0.1
                    if url.scheme() == "http"
                        && !(url.host_str() == Some("localhost")
                            || url.host_str() == Some("127.0.0.1"))
                    {
                        panic!("Only https is allowed except for localhost or 127.0.0.1 with http");
                    }

                    // Get the trimmed URL string without the scheme
                    let trimmed_url = url[url::Position::BeforeHost..].to_string();

                    // Remove the scheme
                    let normalized = if trimmed_url.starts_with("//") {
                        &trimmed_url[2..]
                    } else {
                        &trimmed_url
                    };

                    normalized.to_string()
                } else {
                    Url::parse(&format!("https://{}", domain)).expect("Invalid URL");
                    domain.clone()
                };

                let mut normalized = valid_url.clone();
                if normalized.ends_with("/") {
                    normalized = normalized.trim_end_matches("/").to_string()
                }
                if normalized.ends_with("/did.json") {
                    normalized = normalized.trim_end_matches("/did.json").to_string()
                }
                if normalized.ends_with("/.well-known") {
                    normalized = normalized.trim_end_matches("/.well-known").to_string()
                }

                let encoded_domain = normalized.replace(":", "%3A");
                let encoded_domain = encoded_domain.replace("/", ":");

                let did_web = DidWeb::new(&encoded_domain, public_jwk).unwrap();
                let portable_did = PortableDid {
                    did_uri: did_web.did.uri,
                    document: did_web.document,
                    private_jwks: vec![private_jwk],
                };

                print_portable_did(portable_did, no_indent, json_escape)
            }
            Commands::Dht {
                no_publish,
                no_indent,
                json_escape,
            } => {
                let private_jwk = Ed25519Generator::generate();
                let signer = Ed25519Signer::new(private_jwk.clone());
                let mut identity_key = private_jwk.clone();
                identity_key.d = None;

                let did_dht = DidDht::from_identity_key(identity_key).unwrap();
                if !no_publish {
                    did_dht.publish(Arc::new(signer)).unwrap();
                }

                let portable_did = PortableDid {
                    did_uri: did_dht.did.uri,
                    document: did_dht.document,
                    private_jwks: vec![private_jwk],
                };

                print_portable_did(portable_did, no_indent, json_escape);
            }
        }
    }
}
