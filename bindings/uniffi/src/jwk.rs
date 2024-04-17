use jwk::{Jwk, JwkError};

pub fn compute_thumbprint(jwk: Jwk) -> Result<String, JwkError> {
    jwk.compute_thumbprint()
}
