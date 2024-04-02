use crypto::key::private_key::PrivateKey;
use serde::{Deserialize, Serialize};

use crate::document::Document;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct PortableDID {
    uri: String,

    #[serde(rename = "privateKeys")]
    private_keys: Vec<PrivateKey>,

    document: Document,
}
