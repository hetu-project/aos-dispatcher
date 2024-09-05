use axum::{
  extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
  response::{IntoResponse, Response},
  routing::get,
  Router,
};
// use axum_extra::TypedHeader;

use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;
use msg::{WsMethodMsg, WsResultMsg, WsSendMsg};
use serde_json::json;
use tokio::sync::mpsc;
use util::{connect_to_dispatcher, receive_job_result};
// use futures::{sink::SinkExt, stream::StreamExt};
use std::{borrow::Cow, net::ToSocketAddrs, sync::Arc};
use std::ops::ControlFlow;
use std::{net::SocketAddr, path::PathBuf};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{server::server::SharedState, service::task::DispatchTaskState};

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

  let (tx, mut rx) = mpsc::channel::<Message>(20_000);
  let dispatch_tx;


  {
    let mut server = server.0.write().await;
    server.worker_channels.insert(addr.to_string(), tx.clone());
    dispatch_tx = server.dispatch_task_tx.clone().unwrap();
  }

  ws.on_upgrade(move |socket| {
    handle_socket(socket, addr, tx, rx, dispatch_tx, server.clone())
  })
}
async fn handle_socket(
  mut socket: WebSocket, 
  who: SocketAddr,
  mut tx: mpsc::Sender<Message>,
  mut rx: mpsc::Receiver<Message>,
  dispatch_tx:mpsc::Sender<u32>,
  server: SharedState,
) {
  tracing::info!("{} ws connect", who);
  loop {
      tokio::select!{
        // client send to dispatcher
        Some(msg) = socket.recv() => {
            if let Ok(msg) = msg {
              match &msg {
                  Message::Text(t) => {
                    let command = util::convert_to_msg(t);
                    if let Ok(method_msg) = command {
                      tracing::debug!("Text {:#?}", method_msg);

                       if &method_msg.method == &Some("connect".into()) {
                        let result: WsResultMsg;
                        if let Ok(_) = connect_to_dispatcher(&method_msg, tx.clone(), server.clone()).await {
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
                          tracing::debug!("method {:#?}", method_msg);

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
                          tracing::debug!("method {:#?}", method_msg);

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
                        let result = WsResultMsg {
                          id: method_msg.id.clone(),
                          result: "".into(),
                          address: "".into(),
                          hash: "".into(),
                          signature: "".into(),
                        };
                        tracing::debug!("result {:#?}", method_msg);

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
              // msg
              // Message::Pong(vec![])
            } else {
                // client disconnected
                break;
            };
        },
        Some(msg) = rx.recv() => {
          tracing::debug!("send message to client");
          if socket.send(msg).await.is_err() {
                // client disconnected
                return;
            }
        }

      }
  }
  tracing::info!("{} ws disconnect", who);

}
