use web5::apid::credentials::presentation_definition::PresentationDefinition as InnerPresentationDefinition;

pub struct PresentationDefinition(InnerPresentationDefinition);

impl PresentationDefinition {
    pub fn new(data: InnerPresentationDefinition) -> Self {
        Self { 0: data }
    }

    pub fn select_credentials(&self, vc_jwts: Vec<String>) -> Vec<String> {
        self.0.select_credentials(&vc_jwts).unwrap()
    }
}
