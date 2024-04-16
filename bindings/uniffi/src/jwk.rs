use jwk::{Jwk as InternalJwk, JwkError};

pub struct Jwk(InternalJwk);

impl Jwk {
    pub fn new(
        alg: String,
        kty: String,
        crv: String,
        d: Option<String>,
        x: String,
        y: Option<String>,
    ) -> Self {
        Self(InternalJwk {
            alg,
            kty,
            crv,
            d,
            x,
            y,
        })
    }

    pub fn compute_thumbprint(&self) -> Result<String, JwkError> {
        self.0.compute_thumbprint()
    }
}
