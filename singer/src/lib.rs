pub mod msg_signer;

#[cfg(test)]
mod tests {
    use super::msg_signer::{Keccak256Secp256k1, Signer};
    use secp256k1::{PublicKey, Secp256k1, SecretKey};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct MessageData {
        operator: String,
        address: String,
    }

    #[test]
    fn test_signer() {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&[0x1f; 32]).expect("32 bytes, within curve order");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        let message = MessageData {
            operator: "operator_address".to_string(),
            address: "public_key_address".to_string(),
        };

        let signer = Keccak256Secp256k1;

        let signature = signer.sign_message(&secret_key, &message);
        dbg!(&signature);

        let is_valid = signer.verify_signature(&public_key, &message, &signature);
        dbg!(&is_valid);
        assert_eq!(is_valid, true);
    }
}
