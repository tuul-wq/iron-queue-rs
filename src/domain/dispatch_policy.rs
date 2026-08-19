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
    Quota {
        high: u8,
        normal: u8,
        low: u8,
    },
    /// For aging_step_seconds = 5
    ///
    /// Normal after 5 seconds: 1 + 1 = 2 — ties high.
    /// Normal after 10 seconds: 1 + 2 = 3 — overtakes high.
    /// Low after 10 seconds: 0 + 2 = 2 — ties high.
    /// Low after 15 seconds: 0 + 3 = 3 — overtakes high.
    Aging {
        aging_step_seconds: u8,
    },
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
