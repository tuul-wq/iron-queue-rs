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
    domain::jobs::{Job, JobPriority},
    repository::jobs::JobRepositoryError,
};

#[derive(Serialize)]
pub struct CreateJobResponse {
    id: Uuid,
    name: String,
    priority: JobPriority,
    max_retries: i16,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateJobError {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Repository(#[from] JobRepositoryError),
}

impl IntoResponse for CreateJobError {
    fn into_response(self) -> Response {
        let (code, message) = match &self {
            CreateJobError::Validation(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            CreateJobError::Repository(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        (code, Json(json! { message })).into_response()
    }
}

impl From<Job> for CreateJobResponse {
    fn from(job: Job) -> Self {
        Self {
            id: job.id,
            name: job.name,
            priority: job.priority,
            max_retries: job.max_retries,
            created_at: job.created_at,
        }
    }
}
