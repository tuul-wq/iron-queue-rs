use axum::{Json, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::{Job, JobStatus, Priority};

#[derive(Deserialize)]
pub struct CreateJobRequest {
    name: String,
    kind: String,
    payload: serde_json::Value,
    priority: Priority,
    max_retries: u8,
}

pub async fn create_job(Json(request): Json<CreateJobRequest>) -> (StatusCode, Json<Job>) {
    let job = Job {
        id: Uuid::new_v4(),
        name: request.name,
        kind: request.kind,
        payload: request.payload,
        status: JobStatus::Queued,
        priority: request.priority,
        max_retries: request.max_retries,
    };

    (StatusCode::CREATED, Json(job))
}
