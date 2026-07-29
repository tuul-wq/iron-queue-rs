use serde::Deserialize;
use validator::ValidationErrors;

use crate::domain::{EnqueueJobCommand, JobPayload, JobPriority};

#[derive(Deserialize)]
pub struct CreateJobRequest {
    name: String,
    payload: JobPayload,
    priority: JobPriority,
    max_retries: u8,
}

impl TryFrom<CreateJobRequest> for EnqueueJobCommand {
    type Error = ValidationErrors;

    fn try_from(body: CreateJobRequest) -> Result<Self, Self::Error> {
        EnqueueJobCommand::try_new(body.name, body.payload, body.priority, body.max_retries)
    }
}
