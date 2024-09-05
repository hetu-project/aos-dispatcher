use nostr_sdk::{Event, EventBuilder, Keys, SingleLetterTag, Tag};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::db::pg::model::{JobRequest, Question};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitJob {
    pub from: Value,
    pub job: Value,
    pub verify: Value,
}

#[derive(Serialize, Deserialize)]
pub struct SubmitJobResp {
    pub code: u16,
    pub result: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResultReq {
    pub id: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResultResp {
    pub code: u16,
    pub result: Value,
}


pub struct JobTask {
    event: Event,
    job: Value,
}

impl JobTask {
    pub fn create_with(req: &SubmitJob, keys: &Keys) -> Self {
        let input_tag = Tag::custom(nostr_sdk::TagKind::SingleLetter(SingleLetterTag::lowercase(nostr_sdk::Alphabet::I)), vec!["input"]);
        let tags = vec![input_tag];
        let event_builder = EventBuilder::job_request(nostr_sdk::Kind::JobRequest(5050), tags).unwrap();
        let event = event_builder.to_event(keys).unwrap();
        let job = req.job.clone();
        // let job = json!({

        // });
        Self{
            event,
            job,
        }
    }
}

impl Into<Question> for JobTask {
    fn into(self) -> Question {
        let id = self.event.id.to_string();
        let message = "".into();
        let message_id = self.event.id.to_string();
        let conversation_id = "".into();
        let model = "".into();
        let callback_url = "".into();
        let status = "".into();
        let job_type = "".into();
 
        let q = Question {
            request_id: id,
            message: message,
            message_id: message_id,
            conversation_id: conversation_id,
            model: model,
            callback_url: callback_url,
            status: status,
            job_type: job_type,
            created_at: chrono::Local::now().naive_local(),
        };
        q
    }
}

impl Into<JobRequest> for JobTask {
    fn into(self) -> JobRequest {
        let id = self.event.id.to_string();
        let status = "".into();
        let job_type = "".into();
        let job = self.job;
        let q = JobRequest {
            id: id,
            job,
            status: status,
            job_type: job_type,
            created_at: chrono::Local::now().naive_local(),
        };
        q
    }
}
