use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};

use super::request::CreateJobRequest;
use super::response::{CreateJobError, CreateJobResponse};

use crate::{api::routes::AppState, domain::jobs::CreateJobCommand};

pub async fn create_job(
    State(state): State<AppState>,
    Json(body): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<CreateJobResponse>), CreateJobError> {
    let command = CreateJobCommand::try_from(body)?;

    let job = state.job_repository.create_job(command).await?;

    Ok((StatusCode::CREATED, Json(job.into())))
}

pub async fn get_job(
    State(state): State<AppState>,
    Path(job_id): Path<u16>,
) -> Result<(StatusCode, Json<CreateJobResponse>), Response> {
    todo!("get job {job_id}")
}

pub async fn list_jobs(
    State(state): State<AppState>,
) -> (StatusCode, Json<Vec<CreateJobResponse>>) {
    todo!("list jobs")
}

pub async fn cancel_job(State(state): State<AppState>, Path(job_id): Path<u16>) -> StatusCode {
    todo!("cancel job {job_id}")
}
