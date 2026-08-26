use std::time::Duration;

use crate::domain::{GenerateReportPayload, JobPayload, SendEmailPayload};
use tokio::time::sleep;

const TEMP_FAIL_CODE: &str = "temp_fail";
const PERM_FAIL_CODE: &str = "perm_fail";

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("Job execution will be retried later")]
    Retryable,
    #[error("Job execution permanently failed — {0}")]
    Permament(String),
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

    match email_payload.template_id.as_str() {
        TEMP_FAIL_CODE => {
            tracing::warn!(
                job_kind = "send_email",
                reason = "something went wrong",
                "job execution failed"
            );
            Err(ExecutionError::Retryable)
        }
        PERM_FAIL_CODE => {
            tracing::warn!(
                job_kind = "send_email",
                reason = "email template is unsupported",
                "job execution failed"
            );
            Err(ExecutionError::Permament(
                "Email template is unsupported".to_string(),
            ))
        }
        _ => Ok(()),
    }
}

async fn handle_generate_report(
    report_payload: &GenerateReportPayload,
) -> Result<(), ExecutionError> {
    sleep(Duration::from_secs(1)).await;

    match report_payload.report_type.as_str() {
        TEMP_FAIL_CODE => {
            tracing::warn!(
                job_kind = "generate_report",
                reason = "something went wrong",
                "job execution failed"
            );
            Err(ExecutionError::Retryable)
        }
        PERM_FAIL_CODE => {
            tracing::warn!(
                job_kind = "generate_report",
                reason = "report type is unsupported",
                "job execution failed"
            );
            Err(ExecutionError::Permament(
                "Report type is unsupported".to_string(),
            ))
        }
        _ => Ok(()),
    }
}
