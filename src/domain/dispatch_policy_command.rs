use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Validate)]
pub struct UpdateDispatchPolicyCommand {
    #[validate(range(min = 0, max = 5))]
    test: u8,
}

fn validate_name(name: &str) -> Result<(), ValidationError> {
    if name.trim().is_empty() {
        return Err(ValidationError::new("blank"));
    }

    Ok(())
}

impl UpdateDispatchPolicyCommand {
    pub fn try_new(test: u8) -> Result<Self, ValidationErrors> {
        let command = Self { test };

        command.validate()?;

        Ok(command)
    }

    // pub fn into_new_job(self) -> NewQueuedJob {
    //     NewQueuedJob::new(self.name, self.payload, self.priority, self.max_retries)
    // }
}
