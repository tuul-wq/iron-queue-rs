use std::time::Duration;

use tokio::time::sleep;

use crate::domain::jobs::{GenerateReportPayload, JobPayload, SendEmailPayload};

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("Failed with error {0}")]
    Failed(String),
}

pub async fn execute_job(job: JobPayload) -> Result<(), ExecutionError> {
    match job {
        JobPayload::SendEmail(payload) => handle_send_email(payload).await?,
        JobPayload::GenerateReport(payload) => handle_generate_report(payload).await?,
    }

    Ok(())
}

async fn handle_send_email(email_payload: SendEmailPayload) -> Result<(), ExecutionError> {
    sleep(Duration::from_secs(1)).await;

    if email_payload.template_id == "unsupported" {
        Err(ExecutionError::Failed(
            "Email template is unsupported".to_string(),
        ))
    } else {
        Ok(())
    }
}

async fn handle_generate_report(
    report_payload: GenerateReportPayload,
) -> Result<(), ExecutionError> {
    sleep(Duration::from_secs(1)).await;

    if report_payload.report_type == "unsupported" {
        Err(ExecutionError::Failed(
            "Report template is unsupported".to_string(),
        ))
    } else {
        Ok(())
    }
}
