use std::collections::HashMap;
use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use reqwest::{Client, Url};
use std::time::Duration;
use crate::schema::answers::dsl::*;
use crate::schema::questions;
use crate::schema::answers;
use crate::schema::questions::dsl::{request_id, questions as questions_table};
use crate::schema::answers::dsl::{request_id as answer_request_id};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operator {
    pub worker_name: String,
    pub check_heart_beat: bool,
    pub worker_status: WorkerStatus,
    pub multimodal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStatus {
    pub model_names: Vec<String>,
    pub speed: u16,
    pub queue_length: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorReq {
    pub request_id: String,
    pub node_id: String,
    pub model: String,
    pub prompt: String,
    pub prompt_hash: String,
    pub signature: String,
    pub params: Params,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorResp {
    pub request_id: String,
    pub code: u16,
    pub msg: String,
    pub data: HashMap<String, serde_json::Value>,
}


#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::answers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Answer {
    pub request_id: String,
    pub node_id: String,
    pub model: String,
    pub prompt: String,
    pub answer: String,
    pub attestation: String,
    pub attest_signature: String,
    pub elapsed: i32,
    pub job_type: String,
    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerReq {
    pub request_id: String,
    pub node_id: String,
    pub model: String,
    pub prompt: String,
    pub answer: String,
    pub attestation: String,
    pub attest_signature: String,
    pub elapsed: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerResp {
    pub code: u16,
    pub result: String,
}

#[derive(Serialize, Deserialize)]
pub struct HashRequest {
    pub hash: String,
}

#[derive(Serialize)]
pub struct HashResponse {
    pub sig: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionReq {
    pub message: String,
    pub message_id: String,
    pub conversation_id: String,
    pub model: String,
    pub params: Params,
    pub callback_url: String,
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Question {
    pub request_id: String,
    pub message: String,
    pub message_id: String,
    pub conversation_id: String,
    pub model: String,
    pub callback_url: String,
    pub job_type: String,
    pub status: String,
    #[serde(serialize_with = "serialize_naive_datetime")]
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionResp {
    pub code: u16,
    pub result: QuestionResult,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionResult {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterResp {
    pub code: u16,
    pub result: String,
}

#[derive(Serialize, Deserialize)]
pub struct HeartBeatResp {
    pub exist: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeartBeatReq {
    pub worker_name: String,
    pub queue_length: u16,
}

#[derive(Serialize)]
pub struct ListQuestionsResp {
    pub code: u16,
    pub result: Vec<Question>,
}


#[derive(Serialize)]
pub struct ListAnswersResp {
    pub code: u16,
    pub result: Vec<Answer>,
}

pub fn serialize_naive_datetime<S>(
    date: &NaiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = date.format("%Y-%m-%d %H:%M:%S").to_string();
    serializer.serialize_str(&s)
}

pub fn deserialize_naive_datetime<'de, D>(
    deserializer: D,
) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
}


pub fn list_questions(conn: &mut PgConnection) -> Result<Vec<Question>, diesel::result::Error> {
    questions_table.load::<Question>(conn)
}

pub fn query_latest_question(conn: &mut PgConnection) -> Result<Question, diesel::result::Error>{
    let a = questions::table.order_by(questions::created_at.desc()).first(conn);
    a
}

pub fn list_answers(conn: &mut PgConnection) -> Result<Vec<Answer>, diesel::result::Error> {
    answers.load::<Answer>(conn)
}

pub fn create_question(conn: &mut PgConnection, q_id: String, q_message: String, q_message_id: String, q_conversation_id: String, q_model: String, q_callback_url: String) -> Result<Question, diesel::result::Error> {
    let q = Question {
        request_id: q_id,
        message: q_message,
        message_id: q_message_id,
        conversation_id: q_conversation_id,
        model: q_model,
        callback_url: q_callback_url,
        status: "".into(),
        job_type: "".into(),
        created_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(crate::schema::questions::table)
        .values(&q)
        .returning(Question::as_returning())
        .get_result(conn)
        // .expect("Error saving new question")
}


pub fn create_tee_answer(conn: &mut PgConnection, req: &AnswerReq) -> Result<(), diesel::result::Error> {
    let ans = Answer {
        request_id: req.request_id.clone(),
        node_id: req.node_id.clone(),
        model: req.model.clone(),
        prompt: req.prompt.clone(),
        answer: req.answer.clone(),
        attestation: req.attestation.clone(),
        attest_signature: req.attest_signature.clone(),
        elapsed: req.elapsed as i32,
        job_type: "".into(),
        created_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(crate::schema::answers::table)
        .values(&ans)
        .execute(conn)?;

    Ok(())
}

pub fn get_tee_question(conn: &mut PgConnection) -> Result<Vec<Question>, diesel::result::Error> {
    let r = questions::table
        .select(Question::as_select())
        // .as_query()
        .load(conn);
    r
}

pub fn get_question_by_id(conn: &mut PgConnection, q_id: &str) -> Result<Question, diesel::result::Error> {
    questions::table
        .filter(request_id.eq(q_id))
        .first::<Question>(conn)
}


pub async fn forward_answer_to_callback(ans: &AnswerReq, callback: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Create a payload to send to the callback URL
    let payload = serde_json::json!({
        "request_id": ans.request_id,
        "node_id": ans.node_id,
        "model": ans.model,
        "prompt": ans.prompt,
        "answer": ans.answer,
        "attestation": ans.attestation,
        "attest_signature": ans.attest_signature,
        "elapsed": ans.elapsed,
    });

    let url = Url::parse(&callback)?;

    // Send the POST request to the callback URL
    let response = client
        .post(url)
        .json(&payload)
        .timeout(Duration::from_secs(10))
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Failed to forward answer. Status: {}", response.status()).into())
    }
}


pub fn get_answer_by_id(conn: &mut PgConnection, q_id: &str) -> Result<Option<Answer>, diesel::result::Error> {
    answers::table
        .filter(answer_request_id.eq(q_id))
        .first::<Answer>(conn)
        .optional()
}