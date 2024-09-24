use alloy::{
    primitives::keccak256,
    signers::{local::PrivateKeySigner, Signature, SignerSync},
};
use anyhow::Ok;

pub struct MessageVerify {
    pub singer: PrivateKeySigner,
}

impl MessageVerify {
    pub fn ecdsa_sign(&self, message: &[u8]) -> anyhow::Result<Signature> {
        let signature = self
            .singer
            .sign_message_sync(keccak256(message).as_slice())?;
        Ok(signature)
    }

    pub fn ecdsa_verify(&self, _message: &[u8]) {}
}
