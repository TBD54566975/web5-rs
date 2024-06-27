use crate::errors::Result;
use std::sync::Arc;
use web5::{crypto::jwk::Jwk, dids::data_model::document::Document as InnerDocument};

pub struct Document(pub InnerDocument);

impl Document {
    pub fn new(document: InnerDocument) -> Self {
        Self(document)
    }

    pub fn get_data(&self) -> InnerDocument {
        self.0.clone()
    }

    pub fn find_public_key_jwk(&self, key_id: String) -> Result<Jwk> {
        self.0
            .find_public_key_jwk(key_id)
            .map_err(|e| Arc::new(e.into()))
    }
}
