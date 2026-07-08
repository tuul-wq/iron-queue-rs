use axum::{Json, http::StatusCode};

use crate::domain::job::Job;

pub async fn list_jobs() -> (StatusCode, Json<Vec<Job>>) {
    todo!("list jobs")
}
