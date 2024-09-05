
use nostr_sdk::EventId;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobAnswer {
    pub event_id: EventId,
    pub answer: String,
}