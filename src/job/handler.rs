use axum::{debug_handler, Json};
use axum::extract::State;
use crate::db::pg::model::JobRequest;
use crate::db::pg::util::{get_job_results_by_job_id, get_job_verify_by_user_id};
use crate::job::model::{JobResultReq, JobTask};
use crate::server::server::SharedState;
use serde_json::json;
use crate::db::pg;

use super::model::{JobVerifyReq, SubmitJob};

#[debug_handler]
pub async fn submit_job(State(server): State<SharedState>, Json(req): Json<SubmitJob>) -> Json<serde_json::Value> {
  tracing::debug!("submit job");
  let server = server.0.write().await;
  let dispatch_tx = server.dispatch_task_tx.clone().unwrap();
  let keys = &server.nostr_keys;
  let job = JobTask::create_with(&req, keys);
  let mut question: JobRequest = job.into();
  question.status = String::from("created");
  let mut conn = server.pg.get().expect("Failed to get a connection from pool");
  let q = pg::util::create_job_request(&mut conn, &mut question).expect("Error saving new question");

  // dispatch task

  if let Err(err) = dispatch_tx.send(2).await {
   tracing::error!("dispatch task when submit job {}", err);   
  }

  Json(json!({
    "code": 200,
    "result": q.id,
}))
}

#[debug_handler]
pub async fn query_job_result(State(server): State<SharedState>, Json(req): Json<JobResultReq>) -> Json<serde_json::Value> {
    tracing::info!("query job result {:?}", req);
    let server = server.0.write().await;

    let mut conn = server.pg.get().expect("Failed to get a connection from pool");
    let job_results = get_job_results_by_job_id(&mut conn, &req.job_id.to_string()).unwrap_or_default();

    let response = json!({
        "code": 200,
        "result": job_results,
    });
    Json(response)
}

#[debug_handler]
pub async fn query_job_verify(
    State(server): State<SharedState>,
    Json(req): Json<JobVerifyReq>
) -> Json<serde_json::Value> {
    tracing::info!("query job result {:?}", req);
    let server = server.0.write().await;

    let mut conn = server.pg.get().expect("Failed to get a connection from pool");
    let job_results = get_job_verify_by_user_id(&mut conn, &req.user.to_string()).unwrap_or_default();

    let response = json!({
        "code": 200,
        "result": job_results,
    });
    Json(response)
}