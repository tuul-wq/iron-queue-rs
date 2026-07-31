use tracing::instrument;

use super::DispatchPolicyServiceError;

use crate::{
    domain::{DispatchPolicy, UpdateDispatchPolicyCommand},
    repository::dispatch_policy::DispatchPolicyRepository,
};

#[derive(Clone)]
pub struct DispatchPolicyService {
    repository: DispatchPolicyRepository,
}

impl DispatchPolicyService {
    pub fn new(repository: DispatchPolicyRepository) -> Self {
        Self { repository }
    }

    #[instrument(level = "debug", skip(self, command))]
    pub async fn add_policy(
        &self,
        command: UpdateDispatchPolicyCommand,
    ) -> Result<DispatchPolicy, DispatchPolicyServiceError> {
        self.repository.add_policy(command).await.map_err(|error| {
            tracing::error!(error = %error, "failed to persist dispatch policy");
            DispatchPolicyServiceError::from(error)
        })
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn get_latest_policy(&self) -> Result<DispatchPolicy, DispatchPolicyServiceError> {
        let policy = self.repository.get_latest_policy().await.map_err(|error| {
            tracing::error!(error = %error, "failed to load latest dispatch policy");
            DispatchPolicyServiceError::from(error)
        })?;

        policy.ok_or_else(|| {
            tracing::debug!("dispatch policy not found");
            DispatchPolicyServiceError::NotFound
        })
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn policy_history(&self) -> Result<Vec<DispatchPolicy>, DispatchPolicyServiceError> {
        let policies = self.repository.policy_history().await.map_err(|error| {
            tracing::error!(error = %error, "failed to list dispatch policy history");
            DispatchPolicyServiceError::from(error)
        })?;

        tracing::debug!(
            policy_count = policies.len(),
            "dispatch policy history listed"
        );
        Ok(policies)
    }
}
