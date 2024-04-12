use crate::key::{KeyError, PublicKey};
use jose::jwk::Jwk;

impl PublicKey for Jwk {
    /// Verifies a payload with a given signature using the target [`PublicKey`].
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<(), KeyError> {
        self.verify(payload, signature)?;
        Ok(())
    }

    fn alias(&self) -> Result<String, KeyError> {
        let key_alias = self.thumbprint()?;
        Ok(key_alias)
    }

    fn algorithm(&self) -> Result<String, KeyError> {
        let algorithm = self.algorithm()?;
        Ok(algorithm)
    }

    fn to_json(&self) -> Result<String, KeyError> {
        let json_str = serde_json::to_string(self).map_err(|_| KeyError::SerializationFailed)?;
        Ok(json_str)
    }
}

#[cfg(test)]
mod tests {
    use jose::jwk::{Curve, Jwk};
    use std::sync::Arc;

    #[test]
    fn test_verify() {
        let private_key = Arc::new(Jwk::generate_private_key(Curve::Ed25519.into()).unwrap());
        let payload = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        let public_key = private_key.to_public().unwrap();
        assert!(public_key.verify(payload, &signature).is_ok());
    }

    #[test]
    fn test_verify_failure() {
        let private_key = Arc::new(Jwk::generate_private_key(Curve::Ed25519.into()).unwrap());
        let payload: &[u8] = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        // public_key is unrelated to the private_key used to sign the payload, so it should fail
        let private_key_2 = Arc::new(Jwk::generate_private_key(Curve::Ed25519.into()).unwrap());
        let public_key = private_key_2.to_public().unwrap();
        let verification_warnings = public_key.verify(payload, &signature);
        assert!(verification_warnings.is_err());
    }
}
