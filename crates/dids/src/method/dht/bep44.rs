use std::time::{SystemTime, UNIX_EPOCH};

use byteorder::{BigEndian, WriteBytesExt};
use keys::key_manager::KeyManagerError;

use crate::method::MethodError;

pub fn encode_bep44_message<F>(message: &[u8], sign: F) -> Result<Vec<u8>, MethodError>
where
    F: Fn(Vec<u8>) -> Result<Vec<u8>, KeyManagerError>,
{
    let microseconds_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| {
            MethodError::DidPublishingFailure("Could not get time since epoch".to_string())
        })?
        .as_micros();

    let mut signable =
        format!("3:seqi{}e1:v{}:", microseconds_since_epoch, message.len()).into_bytes();
    signable.extend(message);

    let signature = sign(signable).map_err(|_| {
        MethodError::DidPublishingFailure("Failed to sign BEP44 message".to_string())
    })?;

    let mut seq_bytes = vec![];
    seq_bytes
        .write_u128::<BigEndian>(microseconds_since_epoch)
        .map_err(|_| {
            MethodError::DidPublishingFailure("Failed to write big endian seq".to_string())
        })?;

    let mut encoded = Vec::new();
    encoded.extend(signature);
    encoded.extend(seq_bytes);
    encoded.extend(message);

    Ok(encoded)
}
