use time::OffsetDateTime;

use super::DispatchPolicyRepositoryError;
use crate::domain::{DispatchPolicy, PolicyOption};

#[derive(sqlx::FromRow)]
pub struct DispatchPolicyRow {
    #[sqlx(try_from = "i64")]
    pub id: u64,
    pub policy: serde_json::Value,
    pub created_at: OffsetDateTime,
}

impl TryFrom<DispatchPolicyRow> for DispatchPolicy {
    type Error = DispatchPolicyRepositoryError;

    fn try_from(row: DispatchPolicyRow) -> Result<Self, Self::Error> {
        let policy: PolicyOption = serde_json::from_value(row.policy)
            .map_err(DispatchPolicyRepositoryError::InvalidPolicy)?;

        Ok(Self {
            id: row.id,
            policy,
            created_at: row.created_at,
        })
    }
}
