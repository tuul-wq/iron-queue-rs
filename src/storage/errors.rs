#[derive(Debug, thiserror::Error)]
pub enum JobRepositoryError {
    #[error("Job not found: {0}")]
    NotFound(uuid::Uuid),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}
