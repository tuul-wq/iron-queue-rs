use crate::domain::JobPriority;

use super::strategy::{ClaimRule, JobSelectionStrategy};

pub struct QuotaStrategy {
    schedule: Vec<JobPriority>,
    cursor: usize,
}

impl QuotaStrategy {
    pub fn new(high: u8, normal: u8, low: u8) -> Self {
        let schedule = (0..high)
            .map(|_| JobPriority::High)
            .chain((0..normal).map(|_| JobPriority::Normal))
            .chain((0..low).map(|_| JobPriority::Low))
            .collect();

        Self {
            schedule,
            cursor: 0,
        }
    }
}

impl JobSelectionStrategy for QuotaStrategy {
    fn next_claim_rule(&mut self) -> ClaimRule {
        match self.schedule.get(self.cursor) {
            Some(priority) => ClaimRule::QuotaPriority(priority.clone()),
            None => ClaimRule::QuotaPriority(JobPriority::High),
        }
    }

    fn job_claimed(&mut self) {
        self.cursor = (self.cursor + 1) % self.schedule.len();
    }
}
