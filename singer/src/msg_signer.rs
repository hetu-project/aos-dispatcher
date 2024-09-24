use secp256k1::{ecdsa::Signature, Message, PublicKey, SecretKey};
use serde::Serialize;
use sha3::{Digest, Keccak256};
use thiserror::Error;

pub trait Signer {
    type PrivKey;
    type PubKey;

    fn sign_message<T: Serialize>(&self, privkey: &Self::PrivKey, message: &T) -> String;
    fn verify_signature<T: Serialize>(
        &self,
        pubkey: &Self::PubKey,
        message: &T,
        signature: &str,
    ) -> bool;
}

#[derive(Error, Debug)]
enum SignerError {
    #[error("Error: serde_json error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Clone, Copy, Debug)]
pub struct Keccak256Secp256k1;

impl Keccak256Secp256k1 {
    fn generate_hash<T: Serialize>(message: &T) -> Result<Vec<u8>, SignerError> {
        let serialized_message = serde_json::to_vec(message).expect("Failed to serialize message");
        let mut hasher = Keccak256::new();
        hasher.update(serialized_message);
        Ok(hasher.finalize().to_vec())
        //hex::encode(result)
    }
}

impl Signer for Keccak256Secp256k1 {
    type PrivKey = SecretKey;
    type PubKey = PublicKey;
    fn sign_message<T: Serialize>(&self, privkey: &Self::PrivKey, message: &T) -> String {
        let secp = secp256k1::Secp256k1::new();
        let hash = Self::generate_hash(message).unwrap();
        let msg = Message::from_slice(&hash).expect("hash need 32 bytes");
        let signature = secp.sign_ecdsa(&msg, privkey);
        hex::encode(signature.serialize_compact())
    }

    fn verify_signature<T: Serialize>(
        &self,
        pubkey: &Self::PubKey,
        message: &T,
        signature: &str,
    ) -> bool {
        let secp = secp256k1::Secp256k1::new();
        let hash = Self::generate_hash(message).unwrap();
        let msg = Message::from_slice(&hash).expect("hash need 32 bytes");
        let sig = Signature::from_compact(&hex::decode(signature).expect("Invalid signature"))
            .expect("Invalid format");
        secp.verify_ecdsa(&msg, &sig, pubkey).is_ok()
    }
}
