use axum::{debug_handler, extract::State, Json};
use nostr_sdk::EventId;
use crate::service::nostr::model::JobAnswer;
use crate::tee::model::{Answer, AnswerResp, QuestionReq};
use crate::server::server::SharedState;
use uuid::Uuid;
use chrono::Utc;
use diesel::associations::HasTable;
use diesel::{PgConnection, RunQueryDsl};
use crate::opml::model::*;
use crate::schema::opml_answers::dsl::opml_answers;
use tokio::time::{timeout, Duration};
use std::str::FromStr;
use std::time::Duration as StdDuration;
use tokio::sync::mpsc;


#[debug_handler]
pub async fn opml_question_handler(State(server): State<SharedState>, Json(req): Json<QuestionReq>) -> Json<serde_json::Value> {
    {

        let opml_request = OpmlRequest {
            model: req.model,
            prompt: req.message,
            req_id: req.message_id.clone(),
            callback: req.callback_url.clone(),
        };
        let mut server = server.0.write().await;
        let mut conn = server.pg.get().expect("Failed to get a connection from pool");

        // Store the question in the database
        if let Err(e) = create_opml_question(&mut conn, req.message_id.clone(), &opml_request) {
            tracing::error!("Failed to store OPML question: {:?}", e);
            return Json(serde_json::json!({
            "code": 500,
            "result": "Failed to store OPML question"
        }));
        }

        // Send the request to the OPML server
        if let Err(e) = server.send_opml_request(opml_request).await {
            tracing::error!("Failed to send OPML request: {:?}", e);
            return Json(serde_json::json!({
            "code": 500,
            "result": "Failed to send OPML request"
        }));
        }
    }

    let (tx, mut rx) = mpsc::channel(1);
    {

        let mut server = server.0.write().await;
        // Create a channel for receiving the answer
        server.opml_channels.insert(req.message_id.clone(), tx);
    }

    // Poll the channel for the answer from the callback
    tracing::info!("Waiting for OPML answer, req_id: {}", req.message_id);
    let ret = rx.recv().await;
    tracing::info!("Received OPML answer, req_id: {}", req.message_id);
    match ret {
        Some(answer) => Json(serde_json::json!({
            "code": 200,
            "result": answer
        })),
        None => Json(serde_json::json!({
            "code": 500,
            "result": "Channel closed unexpectedly"
        })),
    }

    // tracing::info!("Waiting for OPML answer, req_id: {}", req.message_id);
    // // Wait for the answer from the callback
    // match tokio::time::timeout(Duration::from_secs(1200), rx.recv()).await {
    //     Ok(Some(answer)) => Json(serde_json::json!({
    //         "code": 200,
    //         "result": answer
    //     })),
    //     Ok(None) => Json(serde_json::json!({
    //         "code": 500,
    //         "result": "Channel closed unexpectedly"
    //     })),
    //     Err(e) => Json(serde_json::json!({
    //         "code": 408,
    //         "result": e.to_string()
    //     })),
    // }
}


#[debug_handler]
pub async fn opml_callback(State(server): State<SharedState>, Json(req): Json<OpmlAnswer>) -> Json<OpmlAnswerResponse> {
    tracing::info!("Handling OPML answer: {:?}", req);

    let mut server = server.0.write().await;
    let mut conn = server.pg.get().expect("Failed to get a connection from pool");
    if let Some(job_status_tx) = server.job_status_tx.clone() {
        job_status_tx.send(JobAnswer {
            event_id: EventId::from_str(&req.req_id).unwrap(),
            answer: req.answer.clone(),
        }).await.unwrap();
    }

    match create_opml_answer(&mut conn, &req) {
        Ok(_) => {
            // Send the answer through the channel if it exists
            if let Some(tx) = server.opml_channels.get(&req.req_id) {
                tracing::info!("Sending OPML answer through channel, req_id: {}", req.req_id);
                if let Err(e) = tx.send(req.clone()).await {
                    tracing::error!("Failed to send OPML answer through channel: {:?}", e);
                }
            }

            let response = OpmlAnswerResponse {
                code: 200,
                result: "OPML answer stored successfully".to_string(),
            };
            Json(response)
        }
        Err(e) => {
            tracing::error!("Failed to store OPML answer: {:?}", e);
            let response = OpmlAnswerResponse {
                code: 500,
                result: "Failed to store OPML answer".to_string(),
            };
            Json(response)
        }
    }
}

pub fn create_opml_answer(conn: &mut PgConnection, opml_answer: &OpmlAnswer) -> Result<(), diesel::result::Error> {
    let new_opml_answer = PgOPMLAnswer {
        req_id: opml_answer.req_id.clone(),
        node_id: opml_answer.node_id.clone(),
        model: opml_answer.model.clone(),
        prompt: opml_answer.prompt.clone(),
        answer: opml_answer.answer.clone(),
        state_root: opml_answer.state_root.clone(),
        created_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(crate::schema::opml_answers::table)
        .values(&new_opml_answer)
        .execute(conn)?;

    Ok(())
}