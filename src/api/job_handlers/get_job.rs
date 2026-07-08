use axum::{Json, extract::Path, http::StatusCode};
use uuid::Uuid;

use crate::domain::job::Job;

pub async fn get_job(Path(job_id): Path<Uuid>) -> (StatusCode, Json<Job>) {
    todo!("get job {job_id}")
}
