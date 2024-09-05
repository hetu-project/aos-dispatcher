use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRegisterParams {
    pub operator: String,
    pub signature: String,
    pub hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRegisterReq {
    pub address: String,
    pub signature: String,
    pub params: OperatorRegisterParams,
}

#[derive(Serialize, Deserialize)]
pub struct OperatorRegisterResp {
    pub code: u16,
    pub message: Value,
    pub result: Value,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorInfoReq {
    pub operator: String,
}
