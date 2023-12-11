use crate::error::Result;
use crypto::key::public_key::PublicKey;

/// Public key struct, exposed to foreign languages.
pub struct PublicKeyFfi(pub(crate) PublicKey);

impl PublicKeyFfi {
    pub fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<Vec<String>> {
        Ok(self.0.verify(payload, signature)?)
    }
}

impl From<PublicKey> for PublicKeyFfi {
    fn from(inner: PublicKey) -> Self {
        Self(inner)
    }
}
