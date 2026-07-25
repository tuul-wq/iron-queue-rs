use tokio::time::{Duration, sleep};
use tracing::{Span, debug, error, field, info, instrument};
use uuid::Uuid;

use crate::{
    repository::jobs::{JobRepository, JobRepositoryError},
    worker::execute_job,
};

pub enum RunnerOutcome {
    Idle,
    Completed { job_id: Uuid },
    Failed { job_id: Uuid },
}

impl std::fmt::Display for RunnerOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunnerOutcome::Idle => write!(f, "idle"),
            RunnerOutcome::Completed { job_id } => write!(f, "completed (job_id: {})", job_id),
            RunnerOutcome::Failed { job_id } => write!(f, "failed (job_id: {})", job_id),
        }
    }
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

pub struct WorkerRunner {
    id: Uuid,
}

impl WorkerRunner {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub async fn run(&self, repository: &JobRepository) -> Result<(), RunnerError> {
        loop {
            match self.run_once(repository).await {
                Ok(RunnerOutcome::Idle) => {
                    debug!(outcome = "idle", "worker run finished");
                }
                Ok(RunnerOutcome::Completed { job_id }) => {
                    info!(outcome = "completed", %job_id, "worker run finished");
                    continue;
                }
                Ok(RunnerOutcome::Failed { job_id }) => {
                    info!(outcome = "failed", %job_id, "worker run finished");
                    continue;
                }
                Err(error) => {
                    debug!(error = %error, "worker failed to run once");
                }
            }

            sleep(Duration::from_secs(1)).await;
        }
    }

    #[instrument(
        level = "info",
        skip(self, repository),
        fields(worker_id = %self.id, job_id = field::Empty)
    )]
    async fn run_once(&self, repository: &JobRepository) -> Result<RunnerOutcome, RunnerError> {
        let job = repository.claim_next(self.id).await.map_err(|source| {
            error!(error = %source, "failed to claim next job");
            RunnerError::ClaimNext { source }
        })?;

        let Some(job) = job else {
            return Ok(RunnerOutcome::Idle);
        };

        Span::current().record("job_id", &field::display(job.id));

        match execute_job(&job.payload).await {
            Ok(_) => {
                repository
                    .mark_completed(job.id, self.id)
                    .await
                    .map_err(|source| {
                        error!(error = %source, "failed to mark job completed");
                        RunnerError::MarkCompleted {
                            job_id: job.id,
                            source,
                        }
                    })?;
                Ok(RunnerOutcome::Completed { job_id: job.id })
            }
            Err(err) => {
                repository
                    .mark_failed(job.id, self.id, &err.to_string())
                    .await
                    .map_err(|source| {
                        error!(error = %source, "failed to mark job failed");
                        RunnerError::MarkFailed {
                            job_id: job.id,
                            source,
                        }
                    })?;
                Ok(RunnerOutcome::Failed { job_id: job.id })
            }
        }
    }
}
