use axum::{Json, http::StatusCode};
use serde_json::json;
use uuid::Uuid;

use crate::domain::{Job, JobStatus, Priority};

pub async fn list_jobs() -> (StatusCode, Json<Vec<Job>>) {
    let jobs = vec![
        Job {
            id: Uuid::new_v4(),
            name: "Name_1".to_string(),
            kind: "send_email".to_string(),
            payload: json!({
                "to": "tuulwq@gmail.com",
                "body": "Hello, test body"
            }),
            status: JobStatus::Queued,
            priority: Priority::Normal,
            max_retries: 3,
        },
        Job {
            id: Uuid::new_v4(),
            name: "Name_2".to_string(),
            kind: "send_email".to_string(),
            payload: json!({
                "to": "tuulwq@gmail.com",
                "body": "Hello, test body"
            }),
            status: JobStatus::Completed,
            priority: Priority::Normal,
            max_retries: 3,
        },
    ];

    (StatusCode::OK, Json(jobs))
}
