use jwk::{Jwk as InternalJwk, JwkError};

pub struct Jwk(pub InternalJwk);

impl From<InternalJwk> for Jwk {
    fn from(value: InternalJwk) -> Self {
        Self::new(value.alg, value.kty, value.crv, value.d, value.x, value.y)
    }
}

impl From<&Jwk> for InternalJwk {
    fn from(value: &Jwk) -> Self {
        Self {
            alg: value.0.alg.clone(),
            kty: value.0.kty.clone(),
            crv: value.0.crv.clone(),
            d: value.0.d.clone(),
            x: value.0.x.clone(),
            y: value.0.y.clone(),
        }
    }
}

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
