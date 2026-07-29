use time::OffsetDateTime;

use super::DispatchPolicyRepositoryError;
use crate::domain::DispatchPolicy;

#[derive(sqlx::FromRow)]
pub struct DispatchPolicyRow {
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TryFrom<DispatchPolicyRow> for DispatchPolicy {
    type Error = DispatchPolicyRepositoryError;

    fn try_from(row: DispatchPolicyRow) -> Result<Self, Self::Error> {
        // let payload: JobPayload = serde_json::from_value(row.payload)
        //     .map_err(DispatchPolicyRepositoryError::InvalidPayload)?;

        todo!();
        // Ok(Self {
        //     created_at: row.created_at,
        //     updated_at: row.updated_at,
        // })
    }
}
