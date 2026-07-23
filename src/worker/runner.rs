use tracing::{Span, error, field, info, instrument, warn};
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

impl std::fmt::Display for RunnerOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunnerOutcome::Idle => write!(f, "idle"),
            RunnerOutcome::Completed => write!(f, "completed"),
            RunnerOutcome::Failed => write!(f, "failed"),
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

    #[instrument(
        level = "info",
        skip(self, repository),
        fields(worker_id = %self.id, job_id = field::Empty)
    )]
    pub async fn run_once(&self, repository: &JobRepository) -> Result<RunnerOutcome, RunnerError> {
        info!("claiming next job");
        let job = repository.claim_next(self.id).await.map_err(|source| {
            error!(error = %source, "failed to claim next job");
            RunnerError::ClaimNext { source }
        })?;

        let Some(job) = job else {
            info!("no queued job available");
            return Ok(RunnerOutcome::Idle);
        };

        Span::current().record("job_id", &field::display(job.id));

        match execute_job(&job.payload).await {
            Ok(_) => {
                info!("job execution completed; marking completed");
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
                info!("job marked completed");
                Ok(RunnerOutcome::Completed)
            }
            Err(err) => {
                warn!(error = %err, "job execution failed; marking failed");
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
                info!("job marked failed");
                Ok(RunnerOutcome::Failed)
            }
        }
    }
}
