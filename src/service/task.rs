use std::collections::HashMap;

use alloy::signers::local::PrivateKeySigner;
use anyhow::{anyhow, Context};
use axum::extract::{ws::Message, FromRef};
use serde_json::json;
use tokio::sync::mpsc;

use crate::{
    consts::{MALICIOUS, SUSPICION},
    db::pg::{
        model::{JobRequest, User},
        util::{
            self, create_user, get_user_by_id, query_new_job_request,
            query_oldest_job_request_with_user,
        },
    },
    message::MessageVerify,
    server::server::SharedState,
    ws::{msg::WsMethodMsg, util::convert_to_msg},
};

#[derive(Debug, Clone, FromRef)]
pub struct DispatchTask {
    pub dispatch_task_tx: mpsc::Sender<u32>,
}

pub async fn dispatch_jobs_to_operators(
    jobs: Vec<JobRequest>,
    operators: &HashMap<String, mpsc::Sender<Message>>,
    position: String,
    signer: PrivateKeySigner,
) {
    let message_verify = MessageVerify { signer };
    for (_j, job) in jobs.iter().enumerate() {
        for (k, tx) in operators {
            tracing::debug!("dispatcher task to {}", k);
            tracing::debug!("dispatcher task  question to {}", k);
            let uuid = uuid::Uuid::new_v4();
            let id = uuid.to_string();
            let mut msg = WsMethodMsg {
                id,
                address: "".into(),
                hash: "".into(),
                signature: "".into(),
                method: Some("dispatch_job".into()),
                params: Some(json!([
                    {
                        "user": job.user,
                        "seed": "",
                        "tag": job.tag,
                        "position": position,
                        "signature": "",
                        "clock": job.clock,
                        "job_id": job.id,
                        "job": job.job,
                    }
                ])),
                result: None,
            };

            let signed_msg = match message_verify.sign_message(&msg) {
                Ok(m) => m,
                Err(err) => {
                    tracing::error!("sign msg error {}", err);
                    continue;
                }
            };
            tracing::debug!("send signed msg {:#?}", signed_msg);
            // use singer::msg_signer::{Keccak256Secp256k1, Signer};
            // let k = Keccak256Secp256k1;
            // let secret_key =  secp256k1::SecretKey::from_slice(&[0xcd; 32]).unwrap();
            // let sig = k.sign_message( &secret_key, &msg);
            // msg.signature = sig;

            // let secp = secp256k1::Secp256k1::new();
            // let public_key = secret_key.public_key(&secp);
            // let address = public_key.serialize().to_lower_hex_string();
            // msg.address = address;
            // let signature = message_verify
            //     .ecdsa_sign(serde_json::to_vec(&msg).unwrap().as_slice())
            //     .unwrap();
            // tracing::debug!("message verify {:#?}", signature.as_bytes());
            let text_msg: Message = signed_msg.into();
            tracing::debug!("msg: {:#?}", text_msg.clone());

            if let Message::Text(text) = text_msg.clone() {
                tracing::debug!("msg text {}", text);
                tracing::debug!("verify message start");

                match convert_to_msg(text.as_str()) {
                    Ok(method_msg) => {
                        let result = MessageVerify::verify_message(&method_msg);

                        tracing::debug!("verify message before send {:#?}", result);
                    },
                    Err(error) => {
                        tracing::error!("verify message before send {:#?}", error);

                    },
                };
 
            }
            if let Err(e) = tx.send(text_msg).await {
                tracing::error!("Send Message {}", e);
            };

            // TODO create job result with status
        }
    }
}

pub async fn dispatch_job(server: SharedState) -> anyhow::Result<()> {
    let server = server.0.write().await;
    let operators = server.operator_channels.iter();
    if operators.len() == 0 {
        return Ok(());
    }

    let mut pool = server.pg.get()?;
    let jobs = query_new_job_request(&mut pool)?;
    let mut job = jobs
        .iter()
        .next()
        .ok_or(anyhow!("there is no job to dispatch"))?
        .clone();

    util::update_job_request_status(&mut pool, &job)
        .context("update job status dispatched error")?;

    if job.tag.as_str() == MALICIOUS || job.tag.as_str() == SUSPICION {
        // let mut old_dispatch_jobs: Vec<JobRequest> = vec![];
        let mut old_dispatch_jobs: Vec<JobRequest> =
            query_oldest_job_request_with_user(&mut pool, job.user.as_str()).unwrap_or_default();
        // old_dispatch_jobs = old_jobs;
        for oj in old_dispatch_jobs.iter_mut() {
            oj.tag = job.tag.clone();
        }

        let user = User {
            id: job.user.clone(),
            name: job.user.clone(),
            address: job.user.clone(),
            status: job.user.clone(),
            tag: job.tag.clone(),
            count: 1,
            created_at: chrono::Local::now().naive_local(),
        };
        let user = create_user(&mut pool, &user)?;
        tracing::debug!("crate user: {}", user.id);
        dispatch_jobs_to_operators(
            old_dispatch_jobs,
            &server.operator_channels,
            "before".into(),
            server.ecdsa_signer.clone(),
        )
        .await;
    }
    let mut position = "";
    if let Ok(mut user) = get_user_by_id(&mut pool, &job.user) {
        if user.tag.as_str() == MALICIOUS || user.tag.as_str() == SUSPICION {
            job.tag = user.tag.clone();
            tracing::debug!("update the job to the tag {}", &job.tag);
            // todo is remove user tag

            if user.count > 10 {
                user.tag = "".into();
                user.count = 0;
            } else {
                user.count = user.count + 1;
            }

            // update user

            position = "after";
            let user = create_user(&mut pool, &user)?;

            tracing::debug!("update user: {}", user.id);
        }
    }
    dispatch_jobs_to_operators(
        vec![job.clone()],
        &server.operator_channels,
        position.into(),
        server.ecdsa_signer.clone(),
    )
    .await;
    Ok(())
}

pub async fn dispatch_task(server: SharedState, mut rx: mpsc::Receiver<u32>) {
    while let Some(i) = rx.recv().await {
        tracing::info!("start dispatch task {}", i);
        match dispatch_job(server.clone()).await {
            Ok(_) => {
                tracing::debug!("dispatch job success");
            }
            Err(err) => {
                tracing::error!("dispatch job success, {}", err);
            }
        };
    }
}
