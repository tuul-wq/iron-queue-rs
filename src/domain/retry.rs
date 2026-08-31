use std::cmp;
use std::time::Duration;

pub struct RetryPolicy {
    /// In seconds
    base_delay: Duration,
    /// In seconds
    max_delay: Duration,
}

pub enum FailureDisposition {
    Retry {
        next_retry_count: u8,
        delay: Duration,
    },
    Terminal,
}

impl RetryPolicy {
    pub fn new(base_delay: Duration, max_delay: Duration) -> Self {
        Self {
            base_delay,
            max_delay,
        }
    }

    pub fn classify(
        &self,
        retry_count: u8,
        max_retries: u8,
        is_retryable: bool,
    ) -> FailureDisposition {
        if !is_retryable || retry_count >= max_retries {
            return FailureDisposition::Terminal;
        }

        FailureDisposition::Retry {
            next_retry_count: retry_count + 1,
            delay: self.calculate_delay(retry_count),
        }
    }

    fn calculate_delay(&self, retry_count: u8) -> Duration {
        let new_delay = self.base_delay * 2u32.pow(retry_count as u32);

        cmp::min(new_delay, self.max_delay)
    }
}
