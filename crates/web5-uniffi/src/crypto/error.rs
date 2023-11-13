#[derive(uniffi::Error, thiserror::Error, Debug)]
pub enum CryptoError {
    #[error("An unknown error occurred")]
    Unknown,
}
