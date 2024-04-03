use crypto::key::private_key::PrivateKey;
use serde::{Deserialize, Serialize};

use crate::document::Document;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct PortableDid {
    pub uri: String,

    #[serde(rename = "privateKeys")]
    pub private_keys: Vec<PrivateKey>,

    pub document: Document,
}
