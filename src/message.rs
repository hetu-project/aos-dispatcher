use std::str::FromStr;

use crate::ws::msg::WsMethodMsg;
use alloy::{
    // primitives::keccak256,
    signers::{local::PrivateKeySigner, Signature, SignerSync},
};

pub struct MessageVerify {
    pub signer: PrivateKeySigner,
}

impl MessageVerify {
    pub fn sign_message(&self, message: &WsMethodMsg) -> anyhow::Result<WsMethodMsg> {
        let mut unsigned_message = message.clone();
        unsigned_message.address = self.signer.address().to_string();
        unsigned_message.signature = String::new();

        let msg = serde_json::to_vec(&unsigned_message)?;
        let signature = self.signer.sign_message_sync(&msg)?;

        let mut signed_message = unsigned_message;
        signed_message.signature = hex::encode(signature.as_bytes());

        Ok(signed_message)
    }

    pub fn verify_message(message: &WsMethodMsg) -> anyhow::Result<bool> {
        let sig = message.signature.as_str();
        let signature = Signature::from_str(sig)?;

        let mut origin_message = message.clone();
        origin_message.signature = String::new();
        origin_message.hash = String::new();

        let msg = serde_json::to_vec(&origin_message)?;
        let origin = signature.recover_address_from_msg(&msg)?;

        let address = message.address.clone();
        let addr = origin.to_string();

        let is_verify = addr.to_lowercase().eq(&address.to_lowercase());
        Ok(is_verify)
    }
}

#[cfg(test)]
mod tests {
    use alloy::signers::local::PrivateKeySigner;

    use crate::ws::msg::WsMethodMsg;

    use super::MessageVerify;

    #[test]
    fn test_verify() {
        let signer = PrivateKeySigner::from_slice(&[0x1f; 32]).expect("singer err");
        let verify = MessageVerify { signer };

        let ws_msg = WsMethodMsg {
            id: "".into(),
            method: None,
            params: None,
            result: None,
            address: "".into(),
            hash: "".into(),
            signature: "".into(),
        };

        let message = verify.sign_message(&ws_msg).expect("sign message error");

        let is_verify = MessageVerify::verify_message(&message).expect("verify message error");

        assert_eq!(is_verify, true, "");

        let mut modify_msg = message.clone();
        modify_msg.method = Some("dispatch_job".into());

        let is_verify = MessageVerify::verify_message(&modify_msg).expect("verify message error");
        assert_eq!(is_verify, false, "");

        let text_msg = r#"
        {"id":"555ebf10-f165-41f6-ae05-918a9232d862","method":"dispatch_job","params":[{"user":"b77f1799de0148c07bc6ef630fb75ac267f31d147cd28797ad145afe72302632","seed":"","tag":"","position":"","signature":"","clock":{"1":"1"},"job_id":"4a16467fc69713bd4ed0b45a4ddab8ed2b69ae14e48236973a75e941f20971fd","job":{"tag":"tee","prompt":"What is AI?","model":"ss","params":{"temperature":1.0,"top_p":0.5,"max_tokens":1024}}}],"result":null,"address":"0x1DdBd306eFFbb5FF29E41398A6a1198Ee6Fb51ce","hash":"","signature":"b5bb37c94af82989f6406de94c921224b00e6cd1ca8079b496507c23f306c56878d5894c8e5ff2fb9296bad91a908a0e6d0ddd516f7286557eaecf36cfcf49401c"}
        "#;
        let receive_msg = serde_json::from_str::<WsMethodMsg>(text_msg).unwrap();
        // dbg!(receive_msg);
        let is_verify = MessageVerify::verify_message(&receive_msg).expect("verify message error");
        assert_eq!(is_verify, true, "");
    }
}
