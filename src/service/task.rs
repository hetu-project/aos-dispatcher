use std::{str::FromStr, sync::Arc};

use axum::extract::FromRef;
use nostr_sdk::EventId;
use serde_json::json;
use tokio::sync::{mpsc, RwLock};

use crate::{
    db::pg::util::query_new_job_request, opml::model::OpmlRequest, server::server::SharedState, service::nostr::{model::JobAnswer, util::query_question}, tee::model::{OperatorReq, Params}, ws::msg::{WsMethodMsg, WsResultMsg, WsSendMsg}
};

#[derive(Debug, Clone)]
pub struct DispatchTaskState(pub(crate) Arc<RwLock<DispatchTask>>);

impl DispatchTaskState {
    pub fn new(tx: mpsc::Sender<u32>) -> Self {
        Self(Arc::new(RwLock::new(DispatchTask {
            dispatch_task_tx: tx,
        })))
    }
}

#[derive(Debug, Clone, FromRef)]
pub struct DispatchTask {
    pub dispatch_task_tx: mpsc::Sender<u32>,
}

pub async fn dispatch_task(server: SharedState, mut rx: mpsc::Receiver<u32>) {
    while let Some(i) = rx.recv().await {
        tracing::info!("start dispatch task {}", i);
        let server = server.0.write().await;
        let mut conn = server
            .pg
            .get()
            .expect("Failed to get a connection from pool");
        let questions = query_new_job_request(&mut conn).unwrap_or_default();
        let dispatch_question = questions.iter().next();

        // dispatch the question by call operator api

        if let Some(q) = dispatch_question {
          let hash = &q.id;
          let signature = server.sign(hash.as_ref());
            // let op_req = OperatorReq {
            //     request_id: q.request_id.clone(),
            //     node_id: "".to_string(),
            //     model: q.model.clone(),
            //     prompt: q.message.clone(),
            //     prompt_hash: hash.to_string(),
            //     signature: signature.to_string(),
            //     params: Params {
            //         temperature: 1.0,
            //         top_p: 0.1,
            //         max_tokens: 1024,
            //     },
            //     r#type: "".to_string(),
            // };

            tracing::debug!("start dispatch task {:#?}", &q.id);

            // send tee

            // let work_name = server
            //     .tee_operator_collections
            //     .keys()
            //     .next();
            //     // .unwrap()
            //     // .clone();
            //   if let Some(work_name) = work_name  {
            //     tracing::debug!("start dispatch task to {:#?}", work_name);

            //     server.send_tee_inductive_task(work_name.clone(), op_req).await;  
            //   }

            // Send the request to the OPML server

            // let opml_request = OpmlRequest {
            //     model: q.model.clone(),
            //     prompt: q.message.clone(),
            //     req_id: q.request_id.clone(),
            //     callback: "".into(),
            // };
            // if let Err(e) = server.send_opml_request(opml_request).await {
            //     tracing::error!("Failed to send OPML request: {:?}", e);
            // }

            // dispatch the question by websocket
            let operators = server.operator_channels.iter();

            for (k, tx) in operators {
                tracing::debug!("dispatcher task to {}", k);
                if let Some(q) = dispatch_question {
                    tracing::debug!("dispatcher task  question to {}", k);

                    let uuid = uuid::Uuid::new_v4();
                    let id = uuid.to_string();
                    let msg = WsMethodMsg {
                        id,
                        address: "".into(),
                        hash: "".into(),
                        signature: "".into(),
                        method: Some("dispatch_job".into()),
                        params: json!([
                            {
                                "user": "", 
                                "seed": "",
                                "signature": "",
                                "job_id": q.id,
                                "job": q.job,
                            }
                        ]),
                        result: None,
                    };
                    tx.send(msg.into()).await.unwrap();

                    // TODO create job result with status
                }
                
            }
        }
    }
}
