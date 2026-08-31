use std::num::TryFromIntError;

use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum JobRepositoryError {
    #[error("Postgres error: {0}")]
    DatabaseFail(#[from] sqlx::Error),

    #[error("Invalid persisted job payload: {0}")]
    InvalidPayload(#[from] serde_json::Error),

    #[error("Invalid schedule delay: {0}")]
    InvalidScheduleDelay(#[from] TryFromIntError),

    #[error("Job transition rejected: {0}")]
    JobTransitionRejected(Uuid),
}
