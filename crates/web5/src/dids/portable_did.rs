use super::data_model::document::Document;
use crate::{
    crypto::jwk::Jwk,
    json::{FromJson, ToJson},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PortableDid {
    #[serde(rename = "uri")]
    pub did_uri: String,
    pub document: Document,
    #[serde(rename = "privateKeys")]
    pub private_jwks: Vec<Jwk>,
}

impl FromJson for PortableDid {}
impl ToJson for PortableDid {}
