use super::aging::AgingStrategy;
use super::quota::QuotaStrategy;
use crate::domain::{JobPriority, PolicyOption};

pub enum ClaimRule {
    QuotaPriority(JobPriority),
    Aging { step_seconds: u8 },
}

pub trait JobSelectionStrategy: Send {
    fn next_claim_rule(&mut self) -> ClaimRule;
    fn job_claimed(&mut self);
}

pub fn strategy_from_policy(policy: &PolicyOption) -> Box<dyn JobSelectionStrategy> {
    match policy {
        PolicyOption::Quota { high, normal, low } => {
            Box::new(QuotaStrategy::new(*high, *normal, *low))
        }
        PolicyOption::Aging { aging_step_seconds } => {
            Box::new(AgingStrategy::new(*aging_step_seconds))
        }
    }
}
