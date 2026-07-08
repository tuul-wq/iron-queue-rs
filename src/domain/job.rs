use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub kind: String,
    pub payload: serde_json::Value,
    pub status: JobStatus,
    pub priority: JobPriority,
    pub retry_count: i16,
    pub max_retries: i16,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "job_priority", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum JobPriority {
    Low,
    Normal,
    High,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "job_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
    Dead,
}
