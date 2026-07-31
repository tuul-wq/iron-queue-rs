use validator::{Validate, ValidationError, ValidationErrors};

use super::{JobPayload, JobPriority, NewQueuedJob};

#[derive(Validate)]
pub struct EnqueueJobCommand {
    #[validate(length(min = 1, max = 50))]
    #[validate(custom(function = "validate_name"))]
    name: String,
    payload: JobPayload,
    priority: JobPriority,
    #[validate(range(min = 0, max = 5))]
    max_retries: u8,
}

fn validate_name(name: &str) -> Result<(), ValidationError> {
    if name.trim().is_empty() {
        return Err(ValidationError::new("Name could not be blank"));
    }

    Ok(())
}

impl EnqueueJobCommand {
    pub fn try_new(
        name: String,
        payload: JobPayload,
        priority: JobPriority,
        max_retries: u8,
    ) -> Result<Self, ValidationErrors> {
        let command = Self {
            name,
            payload,
            priority,
            max_retries,
        };

        command.validate()?;

        Ok(command)
    }

    pub fn into_new_job(self) -> NewQueuedJob {
        NewQueuedJob::new(self.name, self.payload, self.priority, self.max_retries)
    }
}
