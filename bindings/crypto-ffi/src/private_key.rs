use crypto::key::PrivateKey as RustPrivateKey;

pub struct PrivateKey {
    pub(crate) inner: RustPrivateKey,
}

impl From<RustPrivateKey> for PrivateKey {
    fn from(inner: RustPrivateKey) -> Self {
        Self { inner }
    }
}
