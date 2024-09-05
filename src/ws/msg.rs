use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMethodMsg {
  pub id: String,
  pub method: Option<String>,
  pub params: Value,
  pub result: Option<Value>,
  pub address: String,
  pub hash: String,
  pub signature: String,
}

impl  Into<Message> for WsMethodMsg {
  fn into(self) -> Message {
    let json =  serde_json::to_string(&self).unwrap();
    Message::Text(json)
  }
}

 pub enum WsSendMsg {
  Ping
 }


 impl  Into<Message> for WsSendMsg {
    fn into(self) -> Message {
      Message::Text("()".into())
    }
 }



 #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsResultMsg {
  pub id: String,
  pub result: Value,
  pub address: String,
  pub hash: String,
  pub signature: String,
}


impl  Into<Message> for WsResultMsg {
  fn into(self) -> Message {
    let json =  serde_json::to_string(&self).unwrap();
    Message::Text(json)
  }
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectParams {
  pub operator: String,
  pub hash: String,
  pub signature: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResultParams {
  pub id: String,
  pub result: String,
  pub operator: String,
  pub hash: String,
  pub signature: String,
}

