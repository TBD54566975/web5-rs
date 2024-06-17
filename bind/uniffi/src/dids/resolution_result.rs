use web5::apid::dids::resolution_result::ResolutionResult;

pub struct RcbResolutionResult(ResolutionResult);

impl RcbResolutionResult {
    pub fn new(uri: &str) -> Self {
        Self(ResolutionResult::new(uri))
    }

    pub fn get_data(&self) -> ResolutionResult {
        self.0.clone()
    }
}
