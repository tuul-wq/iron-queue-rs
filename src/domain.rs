// ######################################################################
// ################################ Jobs ################################
// ######################################################################

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub kind: String,
    pub payload: serde_json::Value,
    pub status: JobStatus,
    pub priority: Priority,
    pub max_retries: u8,
}

#[derive(Serialize, Deserialize)]
pub enum Priority {
    Low,
    Normal,
    High,
}

#[derive(Serialize, Deserialize)]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}
