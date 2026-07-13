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
        let (name, payload, priority, max_retries) = command.into_parts();

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
            name,
            JobStatus::Queued as JobStatus,
            serde_json::to_value(payload)?,
            priority as JobPriority,
            max_retries,
        )
        .fetch_one(&self.pool)
        .await?;

        Job::try_from(created_job)
    }

    pub async fn get_job(&self, job_id: Uuid) -> Result<Option<Job>, JobRepositoryError> {
        let job = sqlx::query_as!(
            JobRow,
            r#"
              SELECT
                id,
                name,
                payload,
                status AS "status: JobStatus",
                priority AS "priority: JobPriority",
                retry_count,
                max_retries,
                created_at,
                updated_at
              FROM jobs
              WHERE id = $1
          "#,
            job_id
        )
        .fetch_optional(&self.pool)
        .await?;

        job.map(Job::try_from).transpose()
    }

    pub async fn list_jobs(&self) -> Result<Vec<Job>, JobRepositoryError> {
        let jobs = sqlx::query_as!(
            JobRow,
            r#"
            SELECT
              id,
              name,
              payload,
              status AS "status: JobStatus",
              priority AS "priority: JobPriority",
              retry_count,
              max_retries,
              created_at,
              updated_at
            FROM jobs
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        jobs.into_iter().map(Job::try_from).collect()
    }

    pub async fn cancel_job(&self, job_id: Uuid) -> Result<Job, JobRepositoryError> {
        todo!();
    }
}
