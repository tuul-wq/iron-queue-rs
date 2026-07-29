use time::OffsetDateTime;
use uuid::Uuid;

use super::JobRepositoryError;
use crate::domain::{Job, JobPayload, JobPriority, JobStatus};

#[derive(sqlx::FromRow)]
pub struct JobRow {
    pub id: Uuid,
    pub name: String,
    pub payload: serde_json::Value,
    pub status: JobStatus,
    pub priority: JobPriority,
    #[sqlx(try_from = "i16")]
    pub retry_count: u8,
    #[sqlx(try_from = "i16")]
    pub max_retries: u8,
    pub locked_by: Option<Uuid>,
    pub locked_at: Option<OffsetDateTime>,
    pub run_at: Option<OffsetDateTime>,
    pub last_error: Option<String>,
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
            locked_by: row.locked_by,
            locked_at: row.locked_at,
            run_at: row.run_at,
            last_error: row.last_error,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}
