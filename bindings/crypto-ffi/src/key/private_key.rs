use crate::error::Result;
use crypto::key::private_key::PrivateKey as CryptoPrivateKey;

pub struct PrivateKey(pub(crate) CryptoPrivateKey);

impl PrivateKey {
    pub fn new(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self(bincode::deserialize(&bytes)?))
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(&self.0)?)
    }
}

impl From<CryptoPrivateKey> for PrivateKey {
    fn from(inner: CryptoPrivateKey) -> Self {
        Self(inner)
    }
}
