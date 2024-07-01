use crate::errors::Result;
use web5::credentials::presentation_definition::PresentationDefinition as InnerPresentationDefinition;

pub struct PresentationDefinition(pub InnerPresentationDefinition);

impl PresentationDefinition {
    pub fn new(json_serialized_presentation_definition: String) -> Result<Self> {
        let inner_presentation_definition = serde_json::from_str::<InnerPresentationDefinition>(
            &json_serialized_presentation_definition,
        )?;

        Ok(Self(inner_presentation_definition))
    }

    pub fn select_credentials(&self, vc_jwts: &Vec<String>) -> Result<Vec<String>> {
        Ok(self.0.select_credentials(vc_jwts)?)
    }

    pub fn get_json_serialized_presentation_definition(&self) -> Result<String> {
        let json_serialized_presentation_definition = serde_json::to_string(&self.0)?;

        Ok(json_serialized_presentation_definition)
    }
}
