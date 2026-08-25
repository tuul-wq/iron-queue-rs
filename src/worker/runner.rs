use tokio::{
    sync::watch,
    time::{Duration, sleep},
};
use tokio_util::sync::CancellationToken;
use tracing::{Span, field, instrument};
use uuid::Uuid;

use crate::{
    domain::{DispatchPolicy, JobPriority, JobSelectionStrategy, strategy_from_policy},
    repository::jobs::{JobRepository, JobRepositoryError},
};

use super::executor::execute_job;

pub enum RunnerOutcome {
    Idle,
    Completed { job_id: Uuid, priority: JobPriority },
    Failed { job_id: Uuid, priority: JobPriority },
}

impl std::fmt::Display for RunnerOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunnerOutcome::Idle => write!(f, "idle"),
            RunnerOutcome::Completed { job_id, priority } => {
                write!(f, "completed (job_id: {}, priority: {})", job_id, priority)
            }
            RunnerOutcome::Failed { job_id, priority } => {
                write!(f, "failed (job_id: {}, priority: {})", job_id, priority)
            }
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
    job_repo: JobRepository,
    strategy: Box<dyn JobSelectionStrategy>,
    policy_revision: u64,
    policy_rx: watch::Receiver<DispatchPolicy>,
}

impl WorkerRunner {
    pub fn new(job_repo: JobRepository, policy_rx: watch::Receiver<DispatchPolicy>) -> Self {
        let (policy_revision, strategy) = {
            let policy_ref = policy_rx.borrow();
            let strategy = strategy_from_policy(&policy_ref.policy);

            (policy_ref.id, strategy)
        };

        Self {
            id: Uuid::new_v4(),
            job_repo,
            strategy,
            policy_rx,
            policy_revision,
        }
    }

    pub async fn run(&mut self, shutdown_token: &CancellationToken) -> Result<(), RunnerError> {
        tracing::info!(worker_id = %self.id, "worker started");

        loop {
            if shutdown_token.is_cancelled() {
                break;
            }

            self.check_new_strategy();

            match self.run_once().await {
                Ok(RunnerOutcome::Idle) => {
                    tracing::debug!(outcome = "idle", "worker run finished");
                }
                Ok(RunnerOutcome::Completed { job_id, priority }) => {
                    tracing::info!(outcome = "completed", %job_id, priority = %priority, "worker run finished");
                    continue;
                }
                Ok(RunnerOutcome::Failed { job_id, priority }) => {
                    tracing::warn!(outcome = "failed", %job_id, priority = %priority, "worker run finished");
                    continue;
                }
                Err(error) => {
                    tracing::debug!(error = %error, "worker failed to run once");
                }
            }

            if wait_for_next_poll(shutdown_token, 1).await {
                break;
            }
        }

        tracing::info!(worker_id = %self.id, "worker stopped");
        Ok(())
    }

    #[instrument(
        level = "info",
        skip(self),
        fields(worker_id = %self.id, job_id = field::Empty)
    )]
    async fn run_once(&mut self) -> Result<RunnerOutcome, RunnerError> {
        let claim_rule = self.strategy.next_claim_rule();

        let job = self
            .job_repo
            .claim_next(self.id, claim_rule)
            .await
            .map_err(|source| {
                tracing::error!(error = %source, "failed to claim next job");
                RunnerError::ClaimNext { source }
            })?;

        let Some(job) = job else {
            return Ok(RunnerOutcome::Idle);
        };

        self.strategy.job_claimed();

        Span::current().record("job_id", &field::display(job.id));

        match execute_job(&job.payload).await {
            Ok(_) => {
                self.job_repo
                    .mark_completed(job.id, self.id)
                    .await
                    .map_err(|source| {
                        tracing::error!(error = %source, "failed to mark job completed");
                        RunnerError::MarkCompleted {
                            job_id: job.id,
                            source,
                        }
                    })?;
                Ok(RunnerOutcome::Completed {
                    job_id: job.id,
                    priority: job.priority,
                })
            }
            Err(err) => {
                self.job_repo
                    .mark_failed(job.id, self.id, &err.to_string())
                    .await
                    .map_err(|source| {
                        tracing::error!(error = %source, "failed to mark job failed");
                        RunnerError::MarkFailed {
                            job_id: job.id,
                            source,
                        }
                    })?;
                Ok(RunnerOutcome::Failed {
                    job_id: job.id,
                    priority: job.priority,
                })
            }
        }
    }

    fn check_new_strategy(&mut self) {
        let policy_ref = self.policy_rx.borrow();

        if policy_ref.id <= self.policy_revision {
            return;
        }

        tracing::info!(
            old_revision = %self.policy_revision,
            new_revision = %policy_ref.id,
            "policy revision changed, updating strategy"
        );

        self.policy_revision = policy_ref.id;
        self.strategy = strategy_from_policy(&policy_ref.policy);
    }
}

async fn wait_for_next_poll(shutdown_token: &CancellationToken, delay: u64) -> bool {
    tokio::select! {
      _ = shutdown_token.cancelled() => true,
      _ = sleep(Duration::from_secs(delay as u64)) => false,
    }
}
