use crate::error::Result;
use crypto::key::private_key::PrivateKey;

/// Private key struct, exposed to foreign languages.
pub struct PrivateKeyFfi(pub(crate) PrivateKey);

impl PrivateKeyFfi {
    pub fn new(bytes: &[u8]) -> Result<Self> {
        Ok(Self(bincode::deserialize(&bytes)?))
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(&self.0)?)
    }

    pub fn sign(&self, payload: &[u8]) -> Result<Vec<u8>> {
        Ok(self.0.sign(payload)?)
    }

    // TODO: implement to_public. Probably worthwhile for foreign languages to have access to this
}

impl From<PrivateKey> for PrivateKeyFfi {
    fn from(inner: PrivateKey) -> Self {
        Self(inner)
    }
}
