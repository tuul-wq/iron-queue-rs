#[derive(Debug, thiserror::Error)]
pub enum DispatchPolicyRepositoryError {
    #[error("Postgres error: {0}")]
    DatabaseFail(#[from] sqlx::Error),

    #[error("Invalid dispatch_policy payload: {0}")]
    InvalidPolicy(#[from] serde_json::Error),
}
