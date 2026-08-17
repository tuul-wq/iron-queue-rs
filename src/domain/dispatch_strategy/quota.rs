use super::{ClaimRule, JobSelectionStrategy};

pub struct QuotaStrategy {
    high: u8,
    normal: u8,
    low: u8,
}

impl QuotaStrategy {
    pub fn new(high: u8, normal: u8, low: u8) -> Self {
        Self { high, normal, low }
    }
}

impl JobSelectionStrategy for QuotaStrategy {
    fn next_claim_rule(&mut self) -> ClaimRule {
        todo!()
    }
}
