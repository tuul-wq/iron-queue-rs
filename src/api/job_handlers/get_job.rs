use axum::{Json, extract::Path, http::StatusCode};
use serde_json::json;
use uuid::Uuid;

use crate::domain::{Job, JobStatus, Priority};

pub async fn get_job(Path(job_id): Path<Uuid>) -> (StatusCode, Json<Job>) {
    let job = Job {
        id: job_id,
        name: "Name_1".to_string(),
        kind: "send_email".to_string(),
        payload: json!({
            "to": "tuulwq@gmail.com",
            "body": "Hello, test body"
        }),
        status: JobStatus::Queued,
        priority: Priority::Normal,
        max_retries: 3,
    };

    (StatusCode::OK, Json(job))
}
