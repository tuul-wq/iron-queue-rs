use serde::Deserialize;
use validator::{Validate, ValidationErrors};

use crate::domain::jobs::{CreateJobCommand, JobPayload, JobPriority, JobStatus};

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
        let command = CreateJobCommand {
            name: body.name,
            status: JobStatus::Queued,
            payload: body.payload,
            priority: body.priority,
            max_retries: body.max_retries,
        };

        command.validate()?;

        Ok(command)
    }
}
