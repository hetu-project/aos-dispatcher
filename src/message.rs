use std::str::FromStr;

use alloy::{
    primitives::keccak256,
    signers::{local::PrivateKeySigner, Signature, SignerSync},
};
use anyhow::Ok;
use axum::extract::FromRef;

pub struct MessageVerify {
    pub singer: PrivateKeySigner,
}

impl MessageVerify {
    pub fn ecdsa_sign(&self, message: &[u8]) -> anyhow::Result<Signature> {
        let signature = self
            .singer
            .sign_hash_sync(&keccak256(message))?;
        Ok(signature)
    }

    pub fn ecdsa_verify(&self, message: &[u8], signature: &str) -> anyhow::Result<()>  {
        let sign = Signature::from_str(signature)?;

        // let address = sign.recover_address_from_prehash(&keccak256(message))?;
        // address.to_string();
        Ok(())
    }
}
