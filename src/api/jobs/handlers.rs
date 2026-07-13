use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::request::CreateJobRequest;
use super::response::{JobError, JobResponse};

use crate::{api::routes::AppState, domain::jobs::CreateJobCommand};

pub async fn create_job(
    State(state): State<AppState>,
    Json(body): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<JobResponse>), JobError> {
    let command = CreateJobCommand::try_from(body)?;

    let job = state.job_repository.create_job(command).await?;

    Ok((StatusCode::CREATED, Json(job.into())))
}

pub async fn get_job(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<(StatusCode, Json<JobResponse>), JobError> {
    let job = state.job_repository.get_job(job_id).await?;

    if let Some(job) = job {
        return Ok((StatusCode::OK, Json(job.into())));
    };

    Err(JobError::NotFound)
}

pub async fn list_jobs(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<JobResponse>>), JobError> {
    let jobs = state.job_repository.list_jobs().await?;

    Ok((
        StatusCode::OK,
        Json(jobs.into_iter().map(|job| job.into()).collect()),
    ))
}

pub async fn cancel_job(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<StatusCode, JobError> {
    state.job_repository.cancel_job(job_id).await?;

    Ok(StatusCode::OK)
}
