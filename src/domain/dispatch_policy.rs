use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct DispatchPolicy {
    pub id: u64,
    pub policy: PolicyOption,
    pub created_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PolicyOption {
    Quota { high: u8, normal: u8, low: u8 },
    Aging { aging_step_seconds: u8 },
}

#[derive(Serialize)]
pub struct NewDispatchPolicy {
    policy: PolicyOption,
}

impl NewDispatchPolicy {
    pub fn new(policy: PolicyOption) -> Self {
        Self { policy }
    }

    pub fn into_parts(self) -> PolicyOption {
        self.policy
    }
}
