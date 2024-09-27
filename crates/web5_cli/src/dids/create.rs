use clap::Subcommand;
use std::sync::Arc;
use web5::{
    crypto::{
        dsa::ed25519::Ed25519Generator,
        key_managers::{in_memory_key_manager::InMemoryKeyManager, KeyManager},
    },
    dids::{
        methods::{
            did_dht::{DidDht, DidDhtCreateOptions},
            did_jwk::{DidJwk, DidJwkCreateOptions},
            did_web::{DidWeb, DidWebCreateOptions},
        },
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
                let key_manager = Arc::new(InMemoryKeyManager::new());

                let bearer_did = DidJwk::create(Some(DidJwkCreateOptions {
                    key_manager: Some(key_manager.clone()),
                    ..Default::default()
                }))
                .unwrap();

                let portable_did = bearer_did.to_portable_did(key_manager).unwrap();

                print_portable_did(portable_did, no_indent, json_escape);
            }
            Commands::Web {
                domain,
                no_indent,
                json_escape,
            } => {
                let key_manager = Arc::new(InMemoryKeyManager::new());

                let bearer_did = DidWeb::create(
                    domain,
                    Some(DidWebCreateOptions {
                        key_manager: Some(key_manager.clone()),
                        ..Default::default()
                    }),
                )
                .unwrap();

                let portable_did = bearer_did.to_portable_did(key_manager).unwrap();

                print_portable_did(portable_did, no_indent, json_escape)
            }
            Commands::Dht {
                no_publish,
                no_indent,
                json_escape,
            } => {
                let key_manager = Arc::new(InMemoryKeyManager::new());

                let bearer_did = DidDht::create(Some(DidDhtCreateOptions {
                    publish: Some(!no_publish),
                    key_manager: Some(key_manager.clone()),
                    ..Default::default()
                }))
                .unwrap();

                let portable_did = bearer_did.to_portable_did(key_manager).unwrap();

                print_portable_did(portable_did, no_indent, json_escape);
            }
        }
    }
}
