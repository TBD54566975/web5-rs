use super::data_model::document::Document;
use crate::{
    crypto::jwk::Jwk,
    json::{FromJson, ToJson},
};
use serde::{Deserialize, Serialize};

/// Represents a Portable DID (Decentralized Identifier) that includes the DID Document and
/// its associated private keys. This structure is useful for exporting/importing DIDs
/// across different contexts or process boundaries.
#[derive(Serialize, Deserialize, Clone)]
pub struct PortableDid {
    /// The URI of the DID.
    #[serde(rename = "uri")]
    pub did_uri: String,

    /// The DID Document associated with the Portable DID.
    pub document: Document,

    /// The private keys associated with this DID, serialized as JWKs.
    #[serde(rename = "privateKeys")]
    pub private_jwks: Vec<Jwk>,
}

impl FromJson for PortableDid {}
impl ToJson for PortableDid {}
