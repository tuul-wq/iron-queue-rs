use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub payload: JobPayload,
    pub status: JobStatus,
    pub priority: JobPriority,
    pub retry_count: i16,
    pub max_retries: i16,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub struct NewQueuedJob {
    name: String,
    payload: JobPayload,
    priority: JobPriority,
    max_retries: i16,
}

impl NewQueuedJob {
    pub fn new(name: String, payload: JobPayload, priority: JobPriority, max_retries: i16) -> Self {
        Self {
            name,
            payload,
            priority,
            max_retries,
        }
    }

    pub fn into_parts(self) -> (String, JobPayload, JobPriority, i16) {
        (self.name, self.payload, self.priority, self.max_retries)
    }
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "job_priority", rename_all = "snake_case")]
pub enum JobPriority {
    Low,
    Normal,
    High,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "job_status", rename_all = "snake_case")]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
    Dead,
}

#[derive(Serialize, Deserialize)]
pub enum JobPayload {
    SendEmail(SendEmailPayload),
    GenerateReport(GenerateReportPayload),
}

#[derive(Serialize, Deserialize)]
pub struct SendEmailPayload {
    pub to: String,
    pub subject: String,
    pub template_id: String,
    pub variables: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateReportPayload {
    pub report_type: String,
    pub date_range_start: OffsetDateTime,
    pub date_range_end: OffsetDateTime,
    pub format: ReportFormat,
}

#[derive(Serialize, Deserialize)]
pub enum ReportFormat {
    Pdf,
    Csv,
    Excel,
}
