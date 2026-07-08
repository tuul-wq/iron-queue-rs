use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    api::{api_errors::ApiError, routes::AppState},
    domain::job::Job,
};

pub async fn get_job(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Job>), ApiError> {
    todo!("get job {job_id}")
}
