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
use crate::service::nostr::model::JobAnswer;
use crate::tee::model::*;
use crate::server::server::SharedState;
use crate::tee::model::list_questions;
use serde_json::json;
use tokio::sync::mpsc;
use tokio::time::{Duration, Instant};


#[debug_handler]
pub async fn sign(State(server): State<SharedState>, Json(req): Json<HashRequest>) -> Json<HashResponse> {
    let message: &[u8] = req.hash.as_bytes();
    let server = server.0.read().await;
    let signature = server.sign(message);
    let response = HashResponse {
        sig: signature.to_string()
    };
    Json(response)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResponse {
    code: u16,
    result: String,
}

#[debug_handler]
pub async fn tee_question_handler(State(server): State<SharedState>, req: Json<QuestionReq>) -> Json<serde_json::Value> {
    tracing::info!("Handling question {:?}", req);

    let uuid = uuid::Uuid::new_v4();
    let request_id = uuid.to_string();
    {
        let mut server = server.0.write().await;

        let mut conn = server.pg.get().expect("Failed to get a connection from pool");
        let q = create_question(&mut conn, request_id.clone(), req.message.clone(), req.message_id.clone(), req.conversation_id.clone(), req.model.clone(), req.callback_url.clone()).expect("Error saving new question");


        tracing::info!("request_id: {}", request_id);

        let op_req = OperatorReq {
            request_id: q.request_id.clone(),
            node_id: "".to_string(),
            model: req.model.clone(),
            prompt: req.message.clone(),
            prompt_hash: "".to_string(),
            signature: "".to_string(),
            params: req.params.clone(),
            r#type: "".to_string(),
        };


        let work_name = server.tee_operator_collections.keys().next().unwrap().clone();
        server.send_tee_inductive_task(work_name, op_req).await;
    }

    let (tx, mut rx) = mpsc::channel(1);
    {
        let mut server = server.0.write().await;
        server.tee_channels.insert(request_id.clone(), tx);
    }

    // Poll the database for the answer
    match tokio::time::timeout(Duration::from_secs(600), rx.recv()).await {
        Ok(Some(answer)) => {
            Json(json!({
                "code": 200,
                "result": answer
            }))
        }
        _ => {
            // Clean up the channel if we time out
            let mut server = server.0.write().await;
            server.tee_channels.remove(&request_id);

            Json(json!({
                "code": 408,
                "result": "Request timed out"
            }))
        }
    }
}


#[debug_handler]
pub async fn register_worker(State(server): State<SharedState>, Json(req): Json<Operator>) -> Json<RegisterResp> {
    tracing::info!("Registering worker {:?}", req);
    let mut server = server.0.write().await;
    server.add_worker(req.worker_name.clone(), req.check_heart_beat, req.worker_status.clone(), req.multimodal);

    let response = RegisterResp {
        code: 200,
        result: "ok".to_string(),
    };
    Json(response)
}

#[debug_handler]
pub async fn receive_heart_beat(State(server): State<SharedState>, Json(req): Json<HeartBeatReq>) -> Json<HeartBeatResp> {
    tracing::info!("Receiving heart beat {:?}", req);
    let mut server = server.0.write().await;
    let exist = server.tee_operator_collections.contains_key(&req.worker_name);
    let response = HeartBeatResp {
        exist,
    };
    Json(response)
}

#[debug_handler]
pub async fn tee_callback(State(server): State<SharedState>, Json(req): Json<AnswerReq>) -> Json<AnswerResp> {
    tracing::info!("tee_callback function triggered: {:?}", req);

    let server = server.0.read().await;
    let mut conn = server.pg.get().expect("Failed to get a connection from pool");

    if let Some(job_status_tx) = server.job_status_tx.clone() {
        job_status_tx.send(JobAnswer {
            event_id: EventId::from_str(&req.request_id).unwrap(),
            answer: req.answer.clone(),
        }).await.unwrap();
    }

    match create_tee_answer(&mut conn, &req) {
        Ok(_) => {
            // Forward the answer to the callback URL
            if let Some(tx) = server.tee_channels.get(&req.request_id) {
                tracing::info!("Sending answer through channel, request_id: {}", req.request_id);
                if let Err(e) = tx.send(req.clone()).await {
                    tracing::error!("Failed to send OPML answer through channel: {:?}", e);
                }
            }


            let response = AnswerResp {
                code: 200,
                result: "Callback stored successfully".to_string(),
            };
            Json(response)
        }
        Err(e) => {
            tracing::error!("Failed to store callback: {:?}", e);
            let response = AnswerResp {
                code: 500,
                result: "Failed to store callback".to_string(),
            };
            Json(response)
        }
    }
}


pub async fn list_models(State(state): State<SharedState>) -> axum::Json<Vec<String>> {
    let server = state.0.read().await;
    let models: Vec<String> = server.tee_operator_collections.values()
        .flat_map(|operator| operator.worker_status.model_names.clone())
        .collect();

    // Remove duplicates
    let unique_models: Vec<String> = models.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();

    Json(unique_models)
}

pub async fn list_workers(State(state): State<SharedState>) -> axum::Json<Vec<String>> {
    let server = state.0.read().await;
    let workers: Vec<String> = server.tee_operator_collections.keys().cloned().collect();
    Json(workers)
}


pub async fn list_questions_handler(State(server): State<SharedState>) -> Json<ListQuestionsResp> {
    let server = server.0.read().await;
    let mut conn = server.pg.get().expect("Failed to get a connection from pool");

    match list_questions(&mut conn) {
        Ok(questions) => {
            let response = ListQuestionsResp {
                code: 200,
                result: questions,
            };
            Json(response)
        }
        Err(e) => {
            tracing::error!("Failed to list questions: {:?}", e);
            let response = ListQuestionsResp {
                code: 500,
                result: vec![],
            };
            Json(response)
        }
    }
}

pub async fn list_answers_handler(State(server): State<SharedState>) -> Json<ListAnswersResp> {
    let server = server.0.read().await;
    let mut conn = server.pg.get().expect("Failed to get a connection from pool");

    match list_answers(&mut conn) {
        Ok(answers) => {
            let response = ListAnswersResp {
                code: 200,
                result: answers,
            };
            Json(response)
        }
        Err(e) => {
            tracing::error!("Failed to list answers: {:?}", e);
            let response = ListAnswersResp {
                code: 500,
                result: vec![],
            };
            Json(response)
        }
    }
}
pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {error}")),
    )
}