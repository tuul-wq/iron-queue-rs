use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    domain::jobs::{Job, JobPriority, JobStatus},
    service::jobs::JobServiceError,
};

#[derive(Serialize)]
pub struct JobResponse {
    id: Uuid,
    name: String,
    priority: JobPriority,
    status: JobStatus,
    max_retries: u8,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
}

impl From<Job> for JobResponse {
    fn from(job: Job) -> Self {
        Self {
            id: job.id,
            name: job.name,
            priority: job.priority,
            status: job.status,
            max_retries: job.max_retries,
            created_at: job.created_at,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JobError {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Service(#[from] JobServiceError),
}

impl IntoResponse for JobError {
    fn into_response(self) -> Response {
        let (code, message) = match &self {
            JobError::Validation(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            JobError::Service(JobServiceError::NotFound) => {
                (StatusCode::NOT_FOUND, "Job not found".to_string())
            }
            JobError::Service(JobServiceError::Storage(_)) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        (code, Json(json!({ "message": message }))).into_response()
    }
}
