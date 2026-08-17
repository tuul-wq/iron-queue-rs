use super::{ClaimRule, JobSelectionStrategy};

pub struct AgingStrategy {
    aging_step_seconds: u8,
}

impl AgingStrategy {
    pub fn new(aging_step_seconds: u8) -> Self {
        Self { aging_step_seconds }
    }
}

impl JobSelectionStrategy for AgingStrategy {
    fn next_claim_rule(&mut self) -> ClaimRule {
        todo!()
    }
}
