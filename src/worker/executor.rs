use std::time::Duration;

use tokio::time::sleep;
use tracing::warn;

use crate::domain::jobs::{GenerateReportPayload, JobPayload, SendEmailPayload};

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("Job execution will be retried later")]
    Retryable,
    #[error("Job execution permanently failed")]
    Permament,
    #[error("Timed out, will try again later")]
    TimeOut,
    #[error("Unsupported {0}")]
    Unsupported(String),
}

pub async fn execute_job(job_payload: &JobPayload) -> Result<(), ExecutionError> {
    match job_payload {
        JobPayload::SendEmail(payload) => handle_send_email(payload).await?,
        JobPayload::GenerateReport(payload) => handle_generate_report(payload).await?,
    }

    Ok(())
}

async fn handle_send_email(email_payload: &SendEmailPayload) -> Result<(), ExecutionError> {
    sleep(Duration::from_secs(1)).await;

    if email_payload.template_id == "unsupported" {
        warn!(
            job_kind = "send_email",
            reason = "email template is unsupported",
            "job execution failed"
        );
        Err(ExecutionError::Unsupported(
            "Email template is unsupported".to_string(),
        ))
    } else {
        Ok(())
    }
}

async fn handle_generate_report(
    report_payload: &GenerateReportPayload,
) -> Result<(), ExecutionError> {
    sleep(Duration::from_secs(1)).await;

    if report_payload.report_type == "unsupported" {
        warn!(
            job_kind = "generate_report",
            reason = "report type is unsupported",
            "job execution failed"
        );
        Err(ExecutionError::Unsupported(
            "Report type is unsupported".to_string(),
        ))
    } else {
        Ok(())
    }
}
