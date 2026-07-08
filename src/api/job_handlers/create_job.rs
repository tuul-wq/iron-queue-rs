use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;

use crate::{
    api::{api_errors::ApiError, routes::AppState},
    domain::{
        job::{Job, JobPriority},
        new_job::NewJob,
    },
    storage::errors::JobRepositoryError,
};

#[derive(Deserialize)]
pub struct CreateJobRequest {
    name: String,
    kind: String,
    payload: serde_json::Value,
    priority: JobPriority,
    max_retries: i16,
}

pub async fn create_job(
    State(state): State<AppState>,
    Json(request): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<Job>), ApiError> {
    let job = NewJob::try_new(
        request.name,
        request.kind,
        request.payload,
        request.priority,
        request.max_retries,
    )
    .map_err(|err| ApiError::bad_request(err.to_string()))?;

    let job = state
        .job_repository
        .create_job(job)
        .await
        .map_err(|err| match err {
            JobRepositoryError::NotFound(uuid) => {
                ApiError::not_found(format!("Record not found {0}", uuid.to_string()))
            }
            JobRepositoryError::Database(sqlx_err) => ApiError::internal(sqlx_err.to_string()),
        })?;

    Ok((StatusCode::CREATED, Json(job)))
}
