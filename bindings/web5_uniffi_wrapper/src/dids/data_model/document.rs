use web5::{
    dids::data_model::document::Document as InnerDocument,
    json::{FromJson, ToJson},
};

use crate::errors::Result;

pub struct Document(pub InnerDocument);

impl Document {
    pub fn new(document: InnerDocument) -> Self {
        Self(document)
    }

    pub fn get_data(&self) -> InnerDocument {
        self.0.clone()
    }

    pub fn from_json_string(json: String) -> Result<Self> {
        Ok(Self(InnerDocument::from_json_string(&json)?))
    }

    pub fn to_json_string(&self) -> Result<String> {
        Ok(self.0.to_json_string()?)
    }
}
