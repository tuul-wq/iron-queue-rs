#[derive(Debug, thiserror::Error)]
pub enum JobRepositoryError {
    #[error("Postgres error: {0}")]
    DatabaseFail(#[from] sqlx::Error),

    #[error("Invalid persisted job payload: {0}")]
    InvalidPayload(#[from] serde_json::Error),
}
