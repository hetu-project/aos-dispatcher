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
            tracing::debug!("dispatcher job to the operator {}", k);
            let uuid = uuid::Uuid::new_v4();
            let id = uuid.to_string();
            let msg = WsMethodMsg {
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
            let text_msg: Message = signed_msg.into();
            tracing::debug!("start send  success signed msg: {:#?}", text_msg.clone());

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
        tracing::warn!("Operator count is zero");
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
        let mut old_dispatch_jobs: Vec<JobRequest> =
            query_oldest_job_request_with_user(&mut pool, job.user.as_str()).unwrap_or_default();

        tracing::debug!("older dispatch job count is {} ", old_dispatch_jobs.len());
        for oj in old_dispatch_jobs.iter_mut() {
            oj.tag = job.tag.clone();
        }

        let user = User {
            id: job.user.clone(),
            name: job.user.clone(),
            address: job.user.clone(),
            verify_id: "".into(),
            status: "".into(),
            tag: job.tag.clone(),
            count: 1,
            created_at: chrono::Local::now().naive_local(),
        };
        let user = create_user(&mut pool, &user)?;
        tracing::debug!(
            "create or update user: {} with count {}",
            user.id,
            user.count
        );
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
        tracing::debug!("send current job {}", job.id);
        if user.tag.as_str() == MALICIOUS || user.tag.as_str() == SUSPICION {
            job.tag = user.tag.clone();
            tracing::debug!("update the job to the tag {}", &job.tag);
            // todo is remove user tag
            tracing::debug!("the user {} count is {}", user.id, user.count);

            if user.count > 10 {
                user.tag = "".into();
                user.count = 0;
            } else {
                user.count = user.count + 1;
            }

            // update user

            position = "after";
            let user = create_user(&mut pool, &user)?;

            tracing::debug!("update user: {} with count {}", user.id, user.count);
        }
    }

    tracing::debug!("dispatcher current job start");
    dispatch_jobs_to_operators(
        vec![job.clone()],
        &server.operator_channels,
        position.into(),
        server.ecdsa_signer.clone(),
    )
    .await;
    tracing::debug!("dispatcher current job end");
    Ok(())
}

pub async fn dispatch_task(server: SharedState, mut rx: mpsc::Receiver<u32>) {
    while let Some(_) = rx.recv().await {
        tracing::info!("📦------------------------------------------ start dispatch task");
        match dispatch_job(server.clone()).await {
            Ok(_) => {
                tracing::debug!(
                    "📦------------------------------------------ dispatch job success"
                );
            }
            Err(err) => {
                tracing::error!(
                    "📦------------------------------------------  dispatch job error {}",
                    err
                );
            }
        };
    }
}
