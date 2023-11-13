#[derive(uniffi::Error, thiserror::Error, Debug)]
pub enum Web5Error {
    #[error("An unknown error occurred")]
    Unknown,
}
