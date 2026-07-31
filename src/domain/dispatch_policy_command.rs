use validator::{Validate, ValidationError, ValidationErrors};

use super::{NewDispatchPolicy, PolicyOption};

#[derive(Validate)]
pub struct UpdateDispatchPolicyCommand {
    #[validate(custom(function = "validate_policy"))]
    policy: PolicyOption,
}

fn validate_policy(policy: &PolicyOption) -> Result<(), ValidationError> {
    match policy {
        PolicyOption::Quota {
            high: 0,
            normal: 0,
            low: 0,
        } => Err(ValidationError::new("All priorities cannot be zero")),
        PolicyOption::Aging {
            aging_step_seconds: 0,
        } => Err(ValidationError::new("Aging step cannot be zero")),
        _ => Ok(()),
    }
}

impl UpdateDispatchPolicyCommand {
    pub fn try_new(policy: PolicyOption) -> Result<Self, ValidationErrors> {
        let command = Self { policy };

        command.validate()?;

        Ok(command)
    }

    pub fn into_new_policy(self) -> NewDispatchPolicy {
        NewDispatchPolicy::new(self.policy)
    }
}
