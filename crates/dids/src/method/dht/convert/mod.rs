use pkarr::dns::SimpleDnsError;

use crate::method::MethodError;

pub mod document_packet;
mod root_record;
mod service;
mod verification_method;

const DEFAULT_TTL: u32 = 7200; // seconds

/// Errors that can occur when working converting between DID documents and DNS packets.
#[derive(thiserror::Error, Debug)]
pub enum ConvertError {
    #[error(transparent)]
    Dns(#[from] SimpleDnsError),
    #[error("Failure converting service: {0}")]
    Service(String),
    #[error("Failure converting verification method: {0}")]
    VerificationMethod(String),
}

impl From<ConvertError> for MethodError {
    fn from(err: ConvertError) -> Self {
        match err {
            ConvertError::Dns(e) => MethodError::DidPublishingFailure(format!("DNS error: {}", e)),
            ConvertError::Service(msg) => {
                MethodError::DidPublishingFailure(format!("Service conversion failed: {}", msg))
            }
            ConvertError::VerificationMethod(msg) => MethodError::DidPublishingFailure(format!(
                "Verification method conversion failed: {}",
                msg
            )),
        }
    }
}
