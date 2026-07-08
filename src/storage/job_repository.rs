use sqlx::PgPool;

use super::errors::JobRepositoryError;
use crate::domain::{
    job::{Job, JobPriority, JobStatus},
    new_job::NewJob,
};

#[derive(Clone)]
pub struct JobRepository {
    pool: PgPool,
}

impl JobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_job(&self, new_job: NewJob) -> Result<Job, JobRepositoryError> {
        let created_job = sqlx::query_as!(
            Job,
            r#"
              INSERT INTO jobs (name, kind, payload, priority, max_retries)
              VALUES ($1, $2, $3, $4, $5)
              RETURNING
                id,
                name,
                kind,
                payload,
                status AS "status: JobStatus",
                priority AS "priority: JobPriority",
                current_retries,
                max_retries,
                created_at,
                updated_at
          "#,
            new_job.name,
            new_job.kind,
            new_job.payload,
            new_job.priority as JobPriority,
            new_job.max_retries,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_job)
    }

    pub async fn get_job(&self) -> Result<Job, JobRepositoryError> {
        todo!();
    }

    pub async fn list_jobs(&self) -> Result<Vec<Job>, JobRepositoryError> {
        todo!();
    }
}
