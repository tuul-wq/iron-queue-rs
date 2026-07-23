use uuid::Uuid;

use crate::{
    repository::jobs::{JobRepository, JobRepositoryError},
    worker::execute_job,
};

pub enum RunnerOutcome {
    Idle,
    Completed,
    Failed,
}

#[derive(Debug, thiserror::Error)]
pub enum RunnerError {
    #[error("claim_next failed: {source}")]
    ClaimNext { source: JobRepositoryError },
    #[error("mark_completed failed: {source}")]
    MarkCompleted {
        job_id: Uuid,
        source: JobRepositoryError,
    },
    #[error("mark_failed failed: {source}")]
    MarkFailed {
        job_id: Uuid,
        source: JobRepositoryError,
    },
}

pub async fn run_once(
    repository: &JobRepository,
    worker_id: Uuid,
) -> Result<RunnerOutcome, RunnerError> {
    let job = repository
        .claim_next(worker_id)
        .await
        .map_err(|err| RunnerError::ClaimNext { source: err })?;

    let Some(job) = job else {
        return Ok(RunnerOutcome::Idle);
    };

    match execute_job(&job.payload).await {
        Ok(_) => {
            repository
                .mark_completed(job.id, worker_id)
                .await
                .map_err(|err| RunnerError::MarkCompleted {
                    job_id: job.id,
                    source: err,
                })?;
            Ok(RunnerOutcome::Completed)
        }
        Err(err) => {
            repository
                .mark_failed(job.id, worker_id, &err.to_string())
                .await
                .map_err(|err| RunnerError::MarkFailed {
                    job_id: job.id,
                    source: err,
                })?;
            Ok(RunnerOutcome::Failed)
        }
    }
}
