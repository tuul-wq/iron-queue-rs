use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::jobs::{CreateJobCommand, Job, JobPriority, JobStatus},
    repository::jobs::{JobRepositoryError, JobRow},
};

#[derive(Clone)]
pub struct JobRepository {
    pool: PgPool,
}

impl JobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_job(&self, command: CreateJobCommand) -> Result<Job, JobRepositoryError> {
        let payload = serde_json::to_value(command.payload)?;

        let created_job = sqlx::query_as!(
            JobRow,
            r#"
              INSERT INTO jobs (name, status, payload, priority, max_retries)
              VALUES ($1, $2, $3, $4, $5)
              RETURNING
                id,
                name,
                payload,
                status AS "status: JobStatus",
                priority AS "priority: JobPriority",
                retry_count,
                max_retries,
                created_at,
                updated_at
          "#,
            command.name,
            command.status as JobStatus,
            payload,
            command.priority as JobPriority,
            command.max_retries,
        )
        .fetch_one(&self.pool)
        .await?;

        created_job.try_into()
    }

    pub async fn get_job(&self, job_id: Uuid) -> Result<Job, JobRepositoryError> {
        todo!();
        // let job = sqlx::query_as!(Job, r#" SELECT * FROM jobs WHERE id = $1 "#, job_id)
        //     .fetch_optional(&self.pool)
        //     .await?;

        // Ok(job)
    }

    pub async fn list_jobs(&self) -> Result<Vec<Job>, JobRepositoryError> {
        todo!();
    }

    // pub async fn cancel_job(&self) -> Result<Job, JobRepositoryError> {
    //     todo!();
    // }
}
