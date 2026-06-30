use axum::{Json, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::{Job, JobKind, Priority};

#[derive(Deserialize)]
pub struct CreateJobRequest {
    name: String,
    kind: JobKind,
    priority: Priority,
    max_retries: u8,
}

pub async fn create_job(Json(payload): Json<CreateJobRequest>) -> (StatusCode, Json<Job>) {
    let job = Job {
        id: Uuid::new_v4(),
        name: payload.name,
        kind: payload.kind,
        priority: payload.priority,
        max_retries: payload.max_retries,
    };

    (StatusCode::CREATED, Json(job))
}
