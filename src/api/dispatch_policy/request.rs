use serde::Deserialize;
use validator::ValidationErrors;

use crate::domain::{PolicyOption, UpdateDispatchPolicyCommand};

#[derive(Deserialize)]
pub struct UpdateDispatchPolicyRequest {
    policy: PolicyOption,
}

impl TryFrom<UpdateDispatchPolicyRequest> for UpdateDispatchPolicyCommand {
    type Error = ValidationErrors;

    fn try_from(body: UpdateDispatchPolicyRequest) -> Result<Self, Self::Error> {
        UpdateDispatchPolicyCommand::try_new(body.policy)
    }
}
