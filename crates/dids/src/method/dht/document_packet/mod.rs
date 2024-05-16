use simple_dns::SimpleDnsError;

pub mod service;

const DEFAULT_TTL: u32 = 7200; // seconds

/// Errors that can occur when converting between did:dht documents and DNS packets.
#[derive(thiserror::Error, Debug)]
pub enum DocumentPacketError {
    #[error(transparent)]
    Dns(#[from] SimpleDnsError),
    #[error("Could not convert between publicKeyJwk and resource record: {0}")]
    PublicKeyJwk(String),
    #[error("Could not extract fragment from DID url {0}")]
    MissingFragment(String),
    #[error("RData was invalid: {0}")]
    RDataError(String),
}
