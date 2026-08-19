use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{ClaimRule, Job, JobPriority, JobStatus, NewQueuedJob},
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

    pub async fn insert_queued(&self, job: NewQueuedJob) -> Result<Job, JobRepositoryError> {
        let (name, payload, priority, max_retries) = job.into_parts();

        let created_job = sqlx::query_as::<_, JobRow>(
            r#"
              INSERT INTO jobs (name, status, payload, priority, max_retries)
              VALUES ($1, $2, $3, $4, $5)
              RETURNING *
            "#,
        )
        .bind(name)
        .bind(JobStatus::Queued)
        .bind(serde_json::to_value(payload)?)
        .bind(priority)
        .bind(i16::from(max_retries))
        .fetch_one(&self.pool)
        .await?;

        Job::try_from(created_job)
    }

    pub async fn get_job(&self, job_id: Uuid) -> Result<Option<Job>, JobRepositoryError> {
        let job = sqlx::query_as::<_, JobRow>(
            r#"
              SELECT *
              FROM jobs
              WHERE id = $1
            "#,
        )
        .bind(job_id)
        .fetch_optional(&self.pool)
        .await?;

        job.map(Job::try_from).transpose()
    }

    pub async fn list_jobs(&self) -> Result<Vec<Job>, JobRepositoryError> {
        let jobs = sqlx::query_as::<_, JobRow>(
            r#"
              SELECT * FROM jobs
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        jobs.into_iter().map(Job::try_from).collect()
    }

    pub async fn cancel_job(&self, _job_id: Uuid) -> Result<Job, JobRepositoryError> {
        Err(JobRepositoryError::DatabaseFail(
            sqlx::Error::InvalidArgument("Just an Error".to_string()),
        ))
    }

    pub async fn claim_next(
        &self,
        worker_id: Uuid,
        rule: ClaimRule,
    ) -> Result<Option<Job>, JobRepositoryError> {
        match rule {
            ClaimRule::PreferredPriority(priority) => {
                self.claim_next_by_priority(worker_id, priority).await
            }
            ClaimRule::Aging { step_seconds } => {
                self.claim_next_by_aging(worker_id, step_seconds).await
            }
        }
    }

    async fn claim_next_by_priority(
        &self,
        worker_id: Uuid,
        priority: JobPriority,
    ) -> Result<Option<Job>, JobRepositoryError> {
        let job = sqlx::query_as::<_, JobRow>(
            r#"
              WITH new_job AS (
                SELECT id
                FROM jobs
                WHERE status='queued'
                ORDER BY
                    CASE WHEN priority = $1 THEN 0 ELSE 1 END,
                    CASE priority
                        WHEN 'high' THEN 0
                        WHEN 'normal' THEN 1
                        WHEN 'low' THEN 2
                    END,
                    created_at ASC
                FOR UPDATE SKIP LOCKED
                LIMIT 1
              )
              UPDATE jobs
              SET
                status = 'running',
                locked_by = $2,
                locked_at = NOW(),
                updated_at = NOW()
              WHERE id = (SELECT id FROM new_job)
              RETURNING *
            "#,
        )
        .bind(priority)
        .bind(worker_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(job.map(Job::try_from).transpose()?)
    }

    async fn claim_next_by_aging(
        &self,
        worker_id: Uuid,
        step_seconds: u8,
    ) -> Result<Option<Job>, JobRepositoryError> {
        let job = sqlx::query_as::<_, JobRow>(
            r#"
            WITH new_job AS (
              SELECT id
              FROM jobs
              WHERE status='queued'
              ORDER BY
                  (
                      CASE priority
                          WHEN 'low' THEN 0
                          WHEN 'normal' THEN 1
                          WHEN 'high' THEN 2
                      END
                      +
                      FLOOR(EXTRACT(EPOCH FROM (now() - created_at)) / $1)
                  ) DESC,
                  created_at ASC
              FOR UPDATE SKIP LOCKED
              LIMIT 1
            )
            UPDATE jobs
            SET
              status = 'running',
              locked_by = $2,
              locked_at = NOW(),
              updated_at = NOW()
            WHERE id = (SELECT id FROM new_job)
            RETURNING *
          "#,
        )
        .bind(i16::from(step_seconds))
        .bind(worker_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(job.map(Job::try_from).transpose()?)
    }

    pub async fn mark_completed(
        &self,
        job_id: Uuid,
        worker_id: Uuid,
    ) -> Result<(), JobRepositoryError> {
        let result = sqlx::query(
            r#"
            UPDATE jobs
            SET
              status = 'completed',
              locked_by = NULL,
              locked_at = NULL,
              last_error = NULL,
              updated_at = NOW()
            WHERE id = $1 AND locked_by = $2 AND status = 'running'
          "#,
        )
        .bind(job_id)
        .bind(worker_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 1 {
            Ok(())
        } else {
            Err(JobRepositoryError::JobTransitionRejected(job_id))
        }
    }

    pub async fn mark_failed(
        &self,
        job_id: Uuid,
        worker_id: Uuid,
        error: &str,
    ) -> Result<(), JobRepositoryError> {
        let result = sqlx::query(
            r#"
            UPDATE jobs
            SET
              status = 'failed',
              locked_by = NULL,
              locked_at = NULL,
              last_error = $3,
              updated_at = NOW()
            WHERE id = $1 AND locked_by = $2 AND status = 'running'
          "#,
        )
        .bind(job_id)
        .bind(worker_id)
        .bind(error)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 1 {
            Ok(())
        } else {
            Err(JobRepositoryError::JobTransitionRejected(job_id))
        }
    }
}
