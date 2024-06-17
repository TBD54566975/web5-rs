use crate::errors::RcbResult;
use std::sync::Arc;
use web5::apid::credentials::presentation_definition::PresentationDefinition;

pub struct RcbPresentationDefinition(PresentationDefinition);

impl RcbPresentationDefinition {
    pub fn new(pd: PresentationDefinition) -> Self {
        Self(pd)
    }

    pub fn select_credentials(&self, vc_jwts: &Vec<String>) -> RcbResult<Vec<String>> {
        self.0
            .select_credentials(vc_jwts)
            .map_err(|e| Arc::new(e.into()))
    }
}
