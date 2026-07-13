use validator::Validate;

use super::{JobPayload, JobPriority, JobStatus};

#[derive(Validate)]
pub struct CreateJobCommand {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub status: JobStatus,
    pub payload: JobPayload,
    pub priority: JobPriority,
    #[validate(range(min = 0, max = 5))]
    pub max_retries: i16,
}
