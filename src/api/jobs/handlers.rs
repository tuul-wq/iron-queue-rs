use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::instrument;
use uuid::Uuid;

use super::request::CreateJobRequest;
use super::response::{JobError, JobResponse};

use crate::{api::routes::AppState, domain::jobs::EnqueueJobCommand};

#[instrument(skip(state, body))]
pub async fn enqueue_job(
    State(state): State<AppState>,
    Json(body): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<JobResponse>), JobError> {
    let command = EnqueueJobCommand::try_from(body).map_err(|error| {
        tracing::warn!(error = %error, "job enqueue request rejected");
        error
    })?;

    let job = state.job_service.enqueue(command).await?;
    tracing::info!(job_id = %job.id, "job enqueued");

    Ok((StatusCode::CREATED, Json(job.into())))
}

#[instrument(skip(state))]
pub async fn get_job(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<(StatusCode, Json<JobResponse>), JobError> {
    let job = state.job_service.get_job(job_id).await?;

    Ok((StatusCode::OK, Json(job.into())))
}

#[instrument(skip(state))]
pub async fn list_jobs(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<JobResponse>>), JobError> {
    let jobs = state.job_service.list_jobs().await?;

    Ok((
        StatusCode::OK,
        Json(jobs.into_iter().map(|job| job.into()).collect()),
    ))
}

#[instrument(skip(state))]
pub async fn cancel_job(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<StatusCode, JobError> {
    state.job_service.cancel_job(job_id).await?;
    tracing::info!(job_id = %job_id, "job cancelled");

    Ok(StatusCode::OK)
}
