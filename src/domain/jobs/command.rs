use validator::{Validate, ValidationError, ValidationErrors};

use super::{JobPayload, JobPriority};

#[derive(Validate)]
pub struct CreateJobCommand {
    #[validate(length(min = 1, max = 50))]
    #[validate(custom(function = "validate_name"))]
    name: String,
    payload: JobPayload,
    priority: JobPriority,
    #[validate(range(min = 0, max = 5))]
    max_retries: i16,
}

fn validate_name(name: &str) -> Result<(), ValidationError> {
    if name.trim().is_empty() {
        return Err(ValidationError::new("blank"));
    }

    Ok(())
}

impl CreateJobCommand {
    pub fn try_new(
        name: String,
        payload: JobPayload,
        priority: JobPriority,
        max_retries: i16,
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

    pub(crate) fn into_parts(self) -> (String, JobPayload, JobPriority, i16) {
        (self.name, self.payload, self.priority, self.max_retries)
    }
}
