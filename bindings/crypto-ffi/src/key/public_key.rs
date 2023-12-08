use crate::error::Result;
use crypto::key::public_key::PublicKey as CryptoPublicKey;

/// Public key struct, exposed to foreign languages.
pub struct PublicKey(pub(crate) CryptoPublicKey);

impl PublicKey {
    pub fn new(bytes: &[u8]) -> Result<Self> {
        Ok(Self(bincode::deserialize(&bytes)?))
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(&self.0)?)
    }
}

impl From<CryptoPublicKey> for PublicKey {
    fn from(inner: CryptoPublicKey) -> Self {
        Self(inner)
    }
}
