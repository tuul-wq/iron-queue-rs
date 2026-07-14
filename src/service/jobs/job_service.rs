use uuid::Uuid;

use super::error::JobServiceError;

use crate::{
    domain::jobs::{EnqueueJobCommand, Job},
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

    pub async fn enqueue(&self, command: EnqueueJobCommand) -> Result<Job, JobServiceError> {
        self.job_repository
            .insert_queued(command.into_new_job())
            .await
            .map_err(Into::into)
    }

    pub async fn get_job(&self, job_id: Uuid) -> Result<Job, JobServiceError> {
        let job = self.job_repository.get_job(job_id).await?;

        job.ok_or(JobServiceError::NotFound)
    }

    pub async fn list_jobs(&self) -> Result<Vec<Job>, JobServiceError> {
        self.job_repository.list_jobs().await.map_err(Into::into)
    }

    pub async fn cancel_job(&self, job_id: Uuid) -> Result<(), JobServiceError> {
        self.job_repository.cancel_job(job_id).await?;

        Ok(())
    }
}
