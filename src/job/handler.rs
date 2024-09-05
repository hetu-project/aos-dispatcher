use std::borrow::Cow;
use std::str::FromStr;
use axum::{BoxError, debug_handler, extract, Json};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use diesel::{Insertable, Queryable, RunQueryDsl, Selectable};
use nostr_sdk::EventId;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::uuid;
use crate::db::pg::util::{get_answer_by_id, get_job_result_by_id};
use crate::job::model::{JobResultReq, JobResultResp, JobTask};
use crate::service::nostr::model::JobAnswer;
use crate::tee::model::*;
use crate::server::server::SharedState;
use serde_json::json;
use tokio::sync::mpsc;
use tokio::time::{Duration, Instant};
use crate::db::pg;

use super::model::{SubmitJob, SubmitJobResp};

#[debug_handler]
pub async fn submit_job(State(server): State<SharedState>, Json(req): Json<SubmitJob>) -> Json<serde_json::Value> {
  tracing::debug!("submit job");
  let mut server = server.0.write().await;
  let dispatch_tx = server.dispatch_task_tx.clone().unwrap();
  let keys = &server.nostr_keys;
  let job = JobTask::create_with(&req, keys);
  let question = job.into();
  let mut conn = server.pg.get().expect("Failed to get a connection from pool");
  let q = pg::util::create_job_request(&mut conn, &question).expect("Error saving new question");

  // dispatch task
  dispatch_tx.send(2).await.unwrap();

  Json(json!({
    "code": 200,
    "result": q.id,
}))
}

#[debug_handler]
pub async fn query_job_result(State(server): State<SharedState>, Json(req): Json<JobResultReq>) -> Json<serde_json::Value> {
    tracing::info!("query job result {:?}", req);
    let mut server = server.0.write().await;

    let mut conn = server.pg.get().expect("Failed to get a connection from pool");
    let answer = get_job_result_by_id(&mut conn, &req.id.to_string()).unwrap();

    let response = json!({
        "code": 200,
        "result": answer,
    });
    Json(response)
}