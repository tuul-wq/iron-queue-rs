use axum::{Json, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::Job;

#[derive(Deserialize)]
pub struct CreateJobRequest {
    name: String,
}

pub async fn create_job(Json(payload): Json<CreateJobRequest>) -> (StatusCode, Json<Job>) {
    let job = Job {
        id: Uuid::new_v4(),
        name: payload.name,
    };

    (StatusCode::CREATED, Json(job))
}
