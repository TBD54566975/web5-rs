use crate::key::{Key, KeyError, PrivateKey, PublicKey};
use jose::jwk::Jwk;
use std::sync::Arc;

// #[derive(Clone, PartialEq, Debug)]
// pub struct PrivateJwk(pub(crate) Jwk);

impl PrivateKey for Jwk {
    /// Derive a [`PublicKey`] from the target [`PrivateKey`].
    fn to_public(&self) -> Result<Arc<dyn PublicKey>, KeyError> {
        let public_key = self.to_public()?;
        Ok(Arc::new(public_key))
    }

    /// Sign a payload using the target [`PrivateKey`].
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, KeyError> {
        let signature = self.sign(payload)?;
        Ok(signature)
    }
}

impl Key for Jwk {
    fn jwk(&self) -> &Jwk {
        &self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key::KeyType;

    #[test]
    fn test_to_public() {
        let private_key = Arc::new(Jwk::generate_private_key(KeyType::Ed25519.into()).unwrap());
        let public_key = private_key.to_public().unwrap();

        assert_eq!(
            private_key.jwk().parameter("x"),
            public_key.jwk().parameter("x")
        );
        assert_eq!(
            private_key.jwk().parameter("y"),
            public_key.jwk().parameter("y")
        );

        assert!(private_key.jwk().parameter("d").is_some());
        assert!(public_key.jwk().parameter("d").is_none());
    }

    #[test]
    fn test_sign() {
        let private_key = Arc::new(Jwk::generate_private_key(KeyType::Ed25519.into()).unwrap());
        let payload: &[u8] = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        let public_key = private_key.to_public().unwrap();
        assert!(public_key.verify(payload, &signature).is_ok());
    }
}
