use serde::Deserialize;
use validator::ValidationErrors;

use crate::domain::jobs::{CreateJobCommand, JobPayload, JobPriority};

#[derive(Deserialize)]
pub struct CreateJobRequest {
    name: String,
    payload: JobPayload,
    priority: JobPriority,
    max_retries: i16,
}

impl TryFrom<CreateJobRequest> for CreateJobCommand {
    type Error = ValidationErrors;

    fn try_from(body: CreateJobRequest) -> Result<Self, Self::Error> {
        CreateJobCommand::try_new(body.name, body.payload, body.priority, body.max_retries)
    }
}
