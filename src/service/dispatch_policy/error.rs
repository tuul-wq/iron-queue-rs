use crate::repository::dispatch_policy::DispatchPolicyRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum DispatchPolicyServiceError {
    #[error("Dispatch policy not found")]
    NotFound,

    #[error("Unable to access dispatch policy storage")]
    Storage(#[source] DispatchPolicyRepositoryError),
}

impl From<DispatchPolicyRepositoryError> for DispatchPolicyServiceError {
    fn from(error: DispatchPolicyRepositoryError) -> Self {
        Self::Storage(error)
    }
}
