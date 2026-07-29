use time::OffsetDateTime;

pub struct DispatchPolicy {
    policy: JobSelectionStrategy,
    revision: u32,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

pub enum JobSelectionStrategy {
    Quota { high: u8, normal: u8, low: u8 },
    Aging { aging_step_seconds: u8 },
}
