use std::{
    io::Cursor,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

use crate::crypto::CryptoError;
use crate::keys::key::{KeyError, PublicKey};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

/// Minimum size of a bep44 encoded message
/// Signature is 64 bytes and seq is 8 byets
const MIN_MESSAGE_LEN: usize = 72;
/// Maximum size of a bep44 v field
const MAX_V_LEN: usize = 1000;
/// Maximum size a bep44 encoded message
const MAX_MESSAGE_LEN: usize = MAX_V_LEN + MIN_MESSAGE_LEN;

/// Errors that can occur when working with Bep44 messages for did:dht.
#[derive(thiserror::Error, Debug)]
pub enum Bep44EncodingError {
    #[error(transparent)]
    SystemTimeError(#[from] SystemTimeError),
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error("Failure creating DID: {0}")]
    BigEndianError(String),
    #[error(
        "Message must have size between {MIN_MESSAGE_LEN} and {MAX_MESSAGE_LEN} but got size {0}"
    )]
    SizeError(usize),
    #[error(transparent)]
    SignatureError(#[from] KeyError),
}

#[derive(Debug, PartialEq)]
pub struct Bep44Message {
    /// The sequence number of the message, used to ensure the latest version of
    /// the data is retrieved and updated. It's a monotonically increasing number.
    pub seq: u64,
    /// The signature of the message, ensuring the authenticity and integrity
    /// of the data. It's computed over the bencoded sequence number and value.
    pub sig: Vec<u8>,
    /// The actual data being stored or retrieved from the DHT network, typically
    /// encoded in a format suitable for DNS packet representation of a DID Document.
    pub v: Vec<u8>,
}

fn signable(seq: u64, message: &[u8]) -> Vec<u8> {
    let mut signable = format!("3:seqi{}e1:v{}:", seq, message.len()).into_bytes();
    signable.extend(message);
    signable
}

fn encode_seq(seq: u64) -> Result<Vec<u8>, Bep44EncodingError> {
    let mut seq_bytes = vec![];
    seq_bytes.write_u64::<BigEndian>(seq).map_err(|_| {
        Bep44EncodingError::BigEndianError("Failed to write big endian seq".to_string())
    })?;
    Ok(seq_bytes)
}

fn decode_seq(seq_bytes: &[u8]) -> Result<u64, Bep44EncodingError> {
    let mut rdr = Cursor::new(seq_bytes);
    let seq = rdr.read_u64::<BigEndian>().map_err(|_| {
        Bep44EncodingError::BigEndianError("Failed to read big endian seq".to_string())
    })?;
    Ok(seq)
}

/// Represents a BEP44 message, which is used for storing and retrieving data
/// in the Mainline DHT network.
///
/// A BEP44 message is used in the context of the DID DHT method
/// for publishing and resolving DID documents in the DHT network. This type
/// encapsulates the data structure required for such operations in accordance
/// with BEP44.
///
/// See [BEP44 Specification](https://www.bittorrent.org/beps/bep_0044.html)
impl Bep44Message {
    pub fn new<F>(message: &[u8], sign: F) -> Result<Self, Bep44EncodingError>
    where
        F: Fn(Vec<u8>) -> Result<Vec<u8>, KeyError>,
    {
        let message_len = message.len();
        if message_len > MAX_V_LEN {
            return Err(Bep44EncodingError::SizeError(message_len));
        }

        let seq = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let signable = signable(seq, message);
        let sig = sign(signable)?;

        Ok(Bep44Message {
            sig,
            seq,
            v: message.to_vec(),
        })
    }

    pub fn encode(&self) -> Result<Vec<u8>, Bep44EncodingError> {
        let seq_bytes = encode_seq(self.seq)?;

        let mut encoded = Vec::new();
        encoded.extend(self.sig.iter());
        encoded.extend(seq_bytes);
        encoded.extend(self.v.iter());

        Ok(encoded)
    }

    pub fn decode(message_bytes: &[u8]) -> Result<Self, Bep44EncodingError> {
        let message_len = message_bytes.len();
        if !(MIN_MESSAGE_LEN..=MAX_MESSAGE_LEN).contains(&message_len) {
            return Err(Bep44EncodingError::SizeError(message_len));
        }

        let sig = &message_bytes[0..64];
        let seq = decode_seq(&message_bytes[64..72])?;
        let v = &message_bytes[72..];

        Ok(Self {
            seq,
            sig: sig.to_owned(),
            v: v.to_owned(),
        })
    }

    pub fn verify(&self, public_key: &dyn PublicKey) -> Result<(), Bep44EncodingError> {
        let signable = signable(self.seq, &self.v);
        public_key.verify(&signable, &self.sig)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::{ed25519::Ed25519, CurveOperations};
    use crate::keys::key::PrivateKey;

    use super::*;

    #[test]
    fn test_new_verify() {
        let message = "Hello World".as_bytes();

        let private_key = Ed25519::generate().expect("Failed to generate Ed25519 key");

        let result_bep44_message =
            Bep44Message::new(message, |payload| -> Result<Vec<u8>, KeyError> {
                private_key.sign(&payload)
            });
        assert!(result_bep44_message.is_ok());

        let bep44_message = result_bep44_message.unwrap();
        let public_key = private_key
            .to_public()
            .expect("Failed to convert private key to public key");
        let verify_result = bep44_message.verify(public_key.as_ref());
        assert!(verify_result.is_ok());
    }

    #[test]
    fn test_new_message_too_big() {
        let too_big = vec![0; 10_000];
        let error = Bep44Message::new(&too_big, |_| -> Result<Vec<u8>, KeyError> { Ok(vec![]) })
            .expect_err("Should have returned error for malformed signature");

        match error {
            Bep44EncodingError::SizeError(size) => assert_eq!(size, 10_000),
            _ => panic!(),
        }
    }

    #[test]
    fn test_new_sign_fails() {
        let message = "Hello World".as_bytes();

        let error = Bep44Message::new(message, |_| -> Result<Vec<u8>, KeyError> {
            Err(KeyError::CurveNotFound)
        })
        .expect_err("Should have returned error for malformed signature");

        match error {
            Bep44EncodingError::SignatureError(_) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn test_verify_malformed_sig() {
        let message = "Hello World".as_bytes();

        let private_key = Ed25519::generate().expect("Failed to generate Ed25519 key");

        let mut bep44_message =
            Bep44Message::new(message, |payload| -> Result<Vec<u8>, KeyError> {
                private_key.sign(&payload)
            })
            .unwrap();

        // Overwrite sig with malformed signature
        bep44_message.sig = vec![0, 1, 2, 3];
        let public_key = private_key
            .to_public()
            .expect("Failed to convert private key to public key");
        let verify_result = bep44_message.verify(public_key.as_ref());
        assert!(verify_result.is_err());
    }

    #[test]
    fn test_encoded_decode() {
        let message = "Hello World".as_bytes();

        let private_key = Ed25519::generate().expect("Failed to generate Ed25519 key");

        let bep44_message = Bep44Message::new(message, |payload| -> Result<Vec<u8>, KeyError> {
            private_key.sign(&payload)
        })
        .unwrap();

        let encoded = bep44_message
            .encode()
            .expect("Failed to encode bep44 message");
        let decoded = Bep44Message::decode(&encoded).expect("Failed to decode bep44 message");

        assert_eq!(bep44_message, decoded);
    }

    #[test]
    fn test_decode_size_limits() {
        let too_short = vec![1, 2, 3];
        let error = Bep44Message::decode(&too_short)
            .expect_err("Should error because bep44 message is too short");
        match error {
            Bep44EncodingError::SizeError(_) => {}
            _ => panic!(),
        }

        let too_long = vec![0; 2000];
        let error = Bep44Message::decode(&too_long)
            .expect_err("Should error because bep44 message is too long");
        match error {
            Bep44EncodingError::SizeError(_) => {}
            _ => panic!(),
        }
    }
}
