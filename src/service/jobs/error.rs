use crate::repository::jobs::JobRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum JobServiceError {
    #[error("Job not found")]
    NotFound,

    #[error("Unable to access job storage")]
    Storage(#[source] JobRepositoryError),
}

impl From<JobRepositoryError> for JobServiceError {
    fn from(error: JobRepositoryError) -> Self {
        Self::Storage(error)
    }
}
