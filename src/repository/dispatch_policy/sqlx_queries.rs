use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{DispatchPolicy, Job, JobStatus, NewQueuedJob, UpdateDispatchPolicyCommand},
    repository::{
        dispatch_policy::DispatchPolicyRepositoryError,
        jobs::{JobRepositoryError, JobRow},
    },
};

#[derive(Clone)]
pub struct DispatchPolicyRepository {
    pool: PgPool,
}

impl DispatchPolicyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn update_policy(
        &self,
        policy: UpdateDispatchPolicyCommand,
    ) -> Result<DispatchPolicy, DispatchPolicyRepository> {
        todo!();
        // let (name, payload, priority, max_retries) = job.into_parts();

        // let created_job = sqlx::query_as::<_, JobRow>(
        //     r#"
        //       INSERT INTO jobs (name, status, payload, priority, max_retries)
        //       VALUES ($1, $2, $3, $4, $5)
        //       RETURNING *
        //     "#,
        // )
        // .bind(name)
        // .bind(JobStatus::Queued)
        // .bind(serde_json::to_value(payload)?)
        // .bind(priority)
        // .bind(i16::from(max_retries))
        // .fetch_one(&self.pool)
        // .await?;

        // Job::try_from(created_job)
    }

    pub async fn get_policy(
        &self,
    ) -> Result<Option<DispatchPolicy>, DispatchPolicyRepositoryError> {
        todo!();
    }
}
