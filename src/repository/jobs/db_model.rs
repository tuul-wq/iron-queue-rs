use time::OffsetDateTime;
use uuid::Uuid;

use super::JobRepositoryError;
use crate::domain::jobs::{Job, JobPayload, JobPriority, JobStatus};

pub struct JobRow {
    pub id: Uuid,
    pub name: String,
    pub payload: serde_json::Value,
    pub status: JobStatus,
    pub priority: JobPriority,
    pub retry_count: i16,
    pub max_retries: i16,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TryFrom<JobRow> for Job {
    type Error = JobRepositoryError;

    fn try_from(row: JobRow) -> Result<Self, Self::Error> {
        let payload: JobPayload =
            serde_json::from_value(row.payload).map_err(JobRepositoryError::InvalidPayload)?;

        Ok(Self {
            id: row.id,
            name: row.name,
            payload,
            status: row.status,
            priority: row.priority,
            retry_count: row.retry_count,
            max_retries: row.max_retries,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}
