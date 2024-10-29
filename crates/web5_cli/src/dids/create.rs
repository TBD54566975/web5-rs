use crate::utils::warn_if_not_root;
use clap::Subcommand;
use std::sync::Arc;
use url::Url;
use web5::dids::data_model::service::Service;
use web5::{
    crypto::key_managers::in_memory_key_manager::InMemoryKeyManager,
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
        service_endpoint: Option<String>,
        #[arg(long = "service-endpoint-type", default_value = "LinkedDomains")]
        service_endpoint_type: String,
        #[arg(long)]
        no_indent: bool,
        #[arg(long)]
        json_escape: bool,
    },
    Dht {
        #[arg(long)]
        service_endpoint: Option<String>,
        #[arg(long = "service-endpoint-type", default_value = "LinkedDomains")]
        service_endpoint_type: String,
        #[arg(long)]
        no_publish: bool,
        #[arg(long)]
        no_indent: bool,
        #[arg(long)]
        json_escape: bool,
    },
}

fn print_portable_did(
    mut sink: impl std::io::Write,
    portable_did: PortableDid,
    no_indent: &bool,
    json_escape: &bool,
) {
    let mut output_str = match no_indent {
        true => serde_json::to_string(&portable_did).unwrap(),
        false => serde_json::to_string_pretty(&portable_did).unwrap(),
    };

    if *json_escape {
        output_str = output_str.replace('"', "\\\"");
    }

    writeln!(sink, "{}", output_str).unwrap();
}

impl Commands {
    pub async fn command(&self, sink: impl std::io::Write) {
        match self {
            Commands::Jwk {
                no_indent,
                json_escape,
            } => {
                // Check if the current process has root privileges because the InMemoryKeyManager may require root privileges
                warn_if_not_root();
                let key_manager = Arc::new(InMemoryKeyManager::new());

                let bearer_did = DidJwk::create(Some(DidJwkCreateOptions {
                    key_manager: Some(key_manager.clone()),
                    ..Default::default()
                }))
                .unwrap();

                let portable_did = bearer_did.to_portable_did(key_manager).unwrap();

                print_portable_did(sink, portable_did, no_indent, json_escape);
            }
            Commands::Web {
                domain,
                service_endpoint,
                service_endpoint_type,
                no_indent,
                json_escape,
            } => {
                // Check if the current process has root privileges because the InMemoryKeyManager may require root privileges
                warn_if_not_root();
                let key_manager = Arc::new(InMemoryKeyManager::new());

                let mut did_web_create_options = DidWebCreateOptions {
                    key_manager: Some(key_manager.clone()),
                    ..Default::default()
                };

                // Parse the domain to extract the host without the protocol
                let domain_host = match Url::parse(domain) {
                    Ok(parsed_url) => parsed_url.host_str().unwrap_or(domain).to_string(),
                    Err(_) => {
                        // Try adding "https://" if parsing fails
                        match Url::parse(&format!("https://{}", domain)) {
                            Ok(parsed_url) => parsed_url.host_str().unwrap_or(domain).to_string(),
                            Err(_) => {
                                eprintln!("Error: Invalid domain provided.");
                                return;
                            }
                        }
                    }
                };

                // If a service endpoint is provided, add it to the DID document
                if let Some(service_endpoint_url) = service_endpoint {
                    let service = Service {
                        id: format!("did:web:{}#service-1", domain_host),
                        r#type: service_endpoint_type.clone(),
                        service_endpoint: vec![service_endpoint_url.clone()],
                    };
                    did_web_create_options.service = Some(vec![service]);
                }

                let bearer_did = DidWeb::create(domain, Some(did_web_create_options)).unwrap();

                let portable_did = bearer_did.to_portable_did(key_manager).unwrap();

                print_portable_did(sink, portable_did, no_indent, json_escape);
            }
            Commands::Dht {
                service_endpoint,
                service_endpoint_type,
                no_publish,
                no_indent,
                json_escape,
            } => {
                // Check if the current process has root privileges because the InMemoryKeyManager may require root privileges
                warn_if_not_root();
                let key_manager = Arc::new(InMemoryKeyManager::new());

                let mut did_dht_create_options = DidDhtCreateOptions {
                    publish: Some(!no_publish),
                    key_manager: Some(key_manager.clone()),
                    ..Default::default()
                };

                // If a service endpoint is provided, add it to the DID document
                if let Some(service_endpoint_url) = service_endpoint {
                    let service = Service {
                        id: "did:dht:#service-1".to_string(),
                        r#type: service_endpoint_type.clone(),
                        service_endpoint: vec![service_endpoint_url.clone()],
                    };
                    did_dht_create_options.service = Some(vec![service]);
                }

                let bearer_did = DidDht::create(Some(did_dht_create_options)).await.unwrap();

                let portable_did = bearer_did.to_portable_did(key_manager).unwrap();

                print_portable_did(sink, portable_did, no_indent, json_escape);
            }
        }
    }
}
