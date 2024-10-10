use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
// use axum_extra::TypedHeader;

use axum::extract::connect_info::ConnectInfo;
use msg::WsResultMsg;
use serde_json::json;
use tokio::sync::mpsc;
use util::{connect_to_dispatcher, handle_command_msg, receive_job_result};
// use futures::{sink::SinkExt, stream::StreamExt};
use std::net::SocketAddr;

use crate::{message::MessageVerify, server::server::SharedState};

pub mod msg;
pub mod util;

pub async fn handler(
    ws: WebSocketUpgrade,
    State(server): State<SharedState>,
    // State(dispatch_task_state): State<DispatchTaskState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Response {
    tracing::info!("{} connect", addr);

    // send client channel

    let (tx, rx) = mpsc::channel::<Message>(20_000);
    let dispatch_tx;

    {
        let mut server = server.0.write().await;
        // server.worker_channels.insert(addr.to_string(), tx.clone());
        dispatch_tx = server.dispatch_task_tx.clone().unwrap();
    }

    ws.on_upgrade(move |socket| handle_socket(socket, addr, tx, rx, dispatch_tx, server.clone()))
}
async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    tx: mpsc::Sender<Message>,
    mut rx: mpsc::Receiver<Message>,
    _dispatch_tx: mpsc::Sender<u32>,
    server: SharedState,
) {
    let remote_addr = who.to_string();
    tracing::info!("{} ws connect", who);
    let mut connect_operator = None;

    // let message_verify;
    // {
    //     let mut server = server.0.write().await;
    //     let signer = server.ecdsa_signer.clone();
    //     message_verify = MessageVerify { signer };
    // }
    loop {
        tokio::select! {
          // client send to dispatcher
          Some(msg) = socket.recv() => {
              if let Ok(msg) = msg {
                tracing::debug!("{}------------------ handle receive ws message start", "🚀");
                match &msg {
                    Message::Text(t) => {

                      tracing::debug!("receive text message success {}", t);


                      // TODO reactor for handle msg
                      match handle_command_msg(&t, tx.clone()).await {
                          Ok(_) => {
                            tracing::debug!("convert to method msg success");
                          },
                          Err(err) => {
                            tracing::error!("Failed to convert message '{}' to method message: {}", t, err);
                          },
                      };
                      let command = util::convert_to_msg(t);
                      if let Ok(method_msg) = command {
                        if let Ok(b)  = MessageVerify::verify_message(&method_msg) {
                            if !b {
                              tracing::error!("verify error msg {:#?}", method_msg);
                              continue;
                            }
                        };


                         if &method_msg.method == &Some("connect".into()) {
                          let result: WsResultMsg;
                          if let Ok(op) = connect_to_dispatcher(&method_msg, tx.clone(), server.clone(), &remote_addr).await {
                            result = WsResultMsg {
                              id: method_msg.id.clone(),
                              result: json!({
                                "code": 200,
                                "message": "success"
                              }).into(),
                              address: "".into(),
                              hash: "".into(),
                              signature: "".into(),
                            };
                            connect_operator = Some(op);

                          } else {
                            result = WsResultMsg {
                              id: method_msg.id.clone(),
                              result: json!({
                                "code": 500,
                                "message": "error"
                              }).into(),
                              address: "".into(),
                              hash: "".into(),
                              signature: "".into(),
                            };

                          }
                          let _ = socket.send(result.into()).await.is_err();
                         }

                         if &method_msg.method == &Some("job_result".into()) {
                          let result: WsResultMsg;
                          if let Ok(_) = receive_job_result(&method_msg, tx.clone(), server.clone()).await {
                            result = WsResultMsg {
                              id: method_msg.id.clone(),
                              result: json!({
                                "code": 200,
                                "message": "success"
                              }).into(),
                              address: "".into(),
                              hash: "".into(),
                              signature: "".into(),
                            };
                          } else {
                            result = WsResultMsg {
                              id: method_msg.id.clone(),
                              result: json!({
                                "code": 500,
                                "message": "error"
                              }).into(),
                              address: "".into(),
                              hash: "".into(),
                              signature: "".into(),
                            };

                          }
                          let _ = socket.send(result.into()).await.is_err();
                         }

                         if let &Some(_) = &method_msg.result  {
                          let _result = WsResultMsg {
                            id: method_msg.id.clone(),
                            result: "".into(),
                            address: "".into(),
                            hash: "".into(),
                            signature: "".into(),
                          };
                         }

                      }
                    },
                    Message::Binary(b) => {
                      tracing::debug!("Binary {:#?}", b);
                    },
                    Message::Ping(p) => {
                      tracing::debug!("Ping {:#?}", p);

                    },
                    Message::Pong(p) => {
                      tracing::debug!("Pong {:#?}", p);
                    },
                    Message::Close(c) => {
                      tracing::debug!("close {:#?}", c);
                      break;
                    },
                };

                tracing::debug!("🚀------------------ handle receive ws message end");
                // msg
                // Message::Pong(vec![])
              } else {
                  // client disconnected
                  break;
              };
          },
          Some(msg) = rx.recv() => {
            tracing::debug!("send message to operator");
            if socket.send(msg).await.is_err() {
                  // client disconnected
                  tracing::error!("send message to operator error and disconnected");
                  return;
              }
          }

        }
    }
    tracing::info!("operator {:?} on {} ws disconnect", connect_operator, who);
    // clear worker channel
    let mut server = server.0.write().await;
    // server.worker_channels.remove(&who.to_string());
    server.operator_channels.remove(&remote_addr);
}
