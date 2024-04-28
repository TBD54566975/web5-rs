use pkarr::dns::SimpleDnsError;

const DEFAULT_TTL: u32 = 7200; // seconds

/// Errors that can occur when working converting between DID documents and DNS packets.
#[derive(thiserror::Error, Debug)]
pub enum ConvertError {
    #[error(transparent)]
    DnsError(#[from] SimpleDnsError),
    #[error("Failure converting service: {0}")]
    ServiceConvertError(String),
    #[error("Failure converting verification method: {0}")]
    VerificationMethodConvertError(String),
}

pub mod document_packet;
mod root_record;
mod service;
mod verification_method;
