use crate::validators::{ValidationError, required_range, required_string};

use super::job::JobPriority;

pub struct NewJob {
    pub name: String,
    pub kind: String,
    pub payload: serde_json::Value,
    pub priority: JobPriority,
    pub max_retries: i16,
}

impl NewJob {
    const MIN_RETRIES: i16 = 0;
    const MAX_RETRIES: i16 = 3;

    pub fn try_new(
        name: String,
        kind: String,
        payload: serde_json::Value,
        priority: JobPriority,
        max_retries: i16,
    ) -> Result<Self, ValidationError> {
        Ok(Self {
            name: required_string("name", name)?,
            kind: required_string("kind", kind)?,
            payload,
            priority,
            max_retries: required_range(
                "max_retries",
                max_retries,
                Self::MIN_RETRIES,
                Self::MAX_RETRIES,
            )?,
        })
    }
}
