use crypto::CryptoError;
use jwk::JwkError;
use simple_dns::SimpleDnsError;

mod rdata_encoder;
pub mod service;
pub mod verification_method;

const DEFAULT_TTL: u32 = 7200; // seconds

/// Errors that can occur when converting between did:dht documents and DNS packets.
#[derive(thiserror::Error, Debug)]
pub enum DocumentPacketError {
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error(transparent)]
    Dns(#[from] SimpleDnsError),
    #[error(transparent)]
    JwkError(#[from] JwkError),
    #[error("Could not convert between publicKeyJwk and resource record: {0}")]
    PublicKeyJwk(String),
    #[error("Could not extract fragment from DID url {0}")]
    MissingFragment(String),
    #[error("RData was invalid: {0}")]
    RDataError(String),
}
