use std::time::{SystemTime, UNIX_EPOCH};

use nostr_sdk::{Event, EventBuilder, Keys, SingleLetterTag, Tag};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::db::pg::model::JobRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitJob {
    pub from: Value,
    pub job: Value,
    pub user: Option<String>,
    pub tag: Option<String>,
    pub verify: Value,
}

#[derive(Serialize, Deserialize)]
pub struct SubmitJobResp {
    pub code: u16,
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResultReq {
    pub job_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResultResp {
    pub code: u16,
    pub result: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobVerifyReq {
    // pub job_id: String,
    pub user: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobVerifyResp {
    pub code: u16,
    pub result: Value,
}

pub struct JobTask {
    event: Event,
    job: Value,
    submit: SubmitJob,
}

impl JobTask {
    pub fn create_with(req: &SubmitJob, keys: &Keys) -> Self {
        let input_tag = Tag::custom(
            nostr_sdk::TagKind::SingleLetter(SingleLetterTag::lowercase(nostr_sdk::Alphabet::I)),
            vec!["input"],
        );
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let timestamp_tag = Tag::custom(
            nostr_sdk::TagKind::Custom("create_at".into()),
            vec![time.to_string()],
        );
        let tags = vec![input_tag, timestamp_tag];
        let event_builder =
            EventBuilder::job_request(nostr_sdk::Kind::JobRequest(5050), tags).unwrap();
        let event = event_builder.to_event(keys).unwrap();
        let job = req.job.clone();
        // let job = json!({

        // });
        Self {
            event,
            job,
            submit: req.clone(),
        }
    }
}

impl Into<JobRequest> for JobTask {
    fn into(self) -> JobRequest {
        let id = self.event.id.to_string();
        let status = "created".into();
        let job_type = "".into();
        let job = self.job;
        let q = JobRequest {
            id: id,
            job,
            user: self.submit.user.unwrap_or_default(),
            tag: self.submit.tag.unwrap_or_default(),
            clock: json!({
                "1": "1",
            }),
            status: status,
            job_type: job_type,
            created_at: chrono::Local::now().naive_local(),
        };
        q
    }
}
