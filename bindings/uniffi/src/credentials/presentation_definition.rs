use crate::errors::Result;
use std::sync::Arc;
use web5::apid::credentials::presentation_definition::PresentationDefinition as InnerPresentationDefinition;

pub struct PresentationDefinition(pub InnerPresentationDefinition);

impl PresentationDefinition {
    pub fn new(pd: InnerPresentationDefinition) -> Self {
        Self(pd)
    }

    pub fn select_credentials(&self, vc_jwts: &Vec<String>) -> Result<Vec<String>> {
        self.0
            .select_credentials(vc_jwts)
            .map_err(|e| Arc::new(e.into()))
    }
}
