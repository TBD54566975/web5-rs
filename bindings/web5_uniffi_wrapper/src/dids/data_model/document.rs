use crate::errors::ResultOld;
use web5::{crypto::jwk::Jwk, dids::data_model::document::Document as InnerDocument};

pub struct Document(pub InnerDocument);

impl Document {
    pub fn new(document: InnerDocument) -> Self {
        Self(document)
    }

    pub fn get_data(&self) -> InnerDocument {
        self.0.clone()
    }

    pub fn find_public_key_jwk(&self, key_id: String) -> ResultOld<Jwk> {
        Ok(self.0.find_public_key_jwk(key_id)?)
    }
}
