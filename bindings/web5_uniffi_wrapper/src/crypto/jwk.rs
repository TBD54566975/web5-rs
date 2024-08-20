use crate::errors::Result;
use web5::crypto::jwk::Jwk as InnerJwk;

pub struct Jwk(pub InnerJwk);

impl Jwk {
    pub fn new(data: InnerJwk) -> Self {
        Self(data)
    }

    pub fn get_data(&self) -> InnerJwk {
        self.0.clone()
    }

    pub fn compute_thumbprint(&self) -> Result<String> {
        let thumbprint = self.0.compute_thumbprint()?;
        Ok(thumbprint)
    }
}
