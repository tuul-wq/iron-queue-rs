use tracing::instrument;
use uuid::Uuid;

use super::error::JobServiceError;

use crate::{
    domain::{EnqueueJobCommand, Job},
    repository::jobs::JobRepository,
};

#[derive(Clone)]
pub struct JobService {
    job_repository: JobRepository,
}

impl JobService {
    pub fn new(job_repository: JobRepository) -> Self {
        Self { job_repository }
    }

    #[instrument(level = "debug", skip(self, command))]
    pub async fn enqueue(&self, command: EnqueueJobCommand) -> Result<Job, JobServiceError> {
        self.job_repository
            .insert_queued(command.into_new_job())
            .await
            .map_err(|error| {
                tracing::error!(error = %error, "failed to persist queued job");
                JobServiceError::from(error)
            })
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn get_job(&self, job_id: Uuid) -> Result<Job, JobServiceError> {
        let job = self.job_repository.get_job(job_id).await.map_err(|error| {
            tracing::error!(job_id = %job_id, error = %error, "failed to load job");
            JobServiceError::from(error)
        })?;

        job.ok_or_else(|| {
            tracing::debug!(job_id = %job_id, "job not found");
            JobServiceError::NotFound
        })
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn list_jobs(&self) -> Result<Vec<Job>, JobServiceError> {
        let jobs = self.job_repository.list_jobs().await.map_err(|error| {
            tracing::error!(error = %error, "failed to list jobs");
            JobServiceError::from(error)
        })?;

        tracing::debug!(job_count = jobs.len(), "jobs listed");
        Ok(jobs)
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn cancel_job(&self, job_id: Uuid) -> Result<(), JobServiceError> {
        self.job_repository
            .cancel_job(job_id)
            .await
            .map_err(|error| {
                tracing::error!(job_id = %job_id, error = %error, "failed to cancel job");
                JobServiceError::from(error)
            })?;

        Ok(())
    }
}
