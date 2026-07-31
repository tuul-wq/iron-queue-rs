use sqlx::PgPool;

use super::{DispatchPolicyRepositoryError, DispatchPolicyRow};
use crate::domain::{DispatchPolicy, UpdateDispatchPolicyCommand};

#[derive(Clone)]
pub struct DispatchPolicyRepository {
    pool: PgPool,
}

impl DispatchPolicyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn add_policy(
        &self,
        policy: UpdateDispatchPolicyCommand,
    ) -> Result<DispatchPolicy, DispatchPolicyRepositoryError> {
        let policy = policy.into_new_policy().into_parts();

        let created_policy = sqlx::query_as::<_, DispatchPolicyRow>(
            r#"
            INSERT INTO dispatch_policy (policy)
            VALUES ($1)
            RETURNING *
          "#,
        )
        .bind(serde_json::to_value(policy)?)
        .fetch_one(&self.pool)
        .await?;

        DispatchPolicy::try_from(created_policy)
    }

    pub async fn get_latest_policy(
        &self,
    ) -> Result<Option<DispatchPolicy>, DispatchPolicyRepositoryError> {
        let policy = sqlx::query_as::<_, DispatchPolicyRow>(
            r#"
              SELECT * FROM dispatch_policy
              ORDER BY id DESC
              LIMIT 1
          "#,
        )
        .fetch_optional(&self.pool)
        .await?;

        policy.map(DispatchPolicy::try_from).transpose()
    }

    pub async fn policy_history(
        &self,
    ) -> Result<Vec<DispatchPolicy>, DispatchPolicyRepositoryError> {
        let policies = sqlx::query_as::<_, DispatchPolicyRow>(
            r#"
              SELECT * FROM dispatch_policy
          "#,
        )
        .fetch_all(&self.pool)
        .await?;

        policies.into_iter().map(DispatchPolicy::try_from).collect()
    }
}
