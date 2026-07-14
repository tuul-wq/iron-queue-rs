use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::request::CreateJobRequest;
use super::response::{JobError, JobResponse};

use crate::{api::routes::AppState, domain::jobs::EnqueueJobCommand};

pub async fn enqueue_job(
    State(state): State<AppState>,
    Json(body): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<JobResponse>), JobError> {
    let command = EnqueueJobCommand::try_from(body)?;

    let job = state.job_service.enqueue(command).await?;

    Ok((StatusCode::CREATED, Json(job.into())))
}

pub async fn get_job(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<(StatusCode, Json<JobResponse>), JobError> {
    let job = state.job_service.get_job(job_id).await?;

    Ok((StatusCode::OK, Json(job.into())))
}

pub async fn list_jobs(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<JobResponse>>), JobError> {
    let jobs = state.job_service.list_jobs().await?;

    Ok((
        StatusCode::OK,
        Json(jobs.into_iter().map(|job| job.into()).collect()),
    ))
}

pub async fn cancel_job(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<StatusCode, JobError> {
    state.job_service.cancel_job(job_id).await?;

    Ok(StatusCode::OK)
}
