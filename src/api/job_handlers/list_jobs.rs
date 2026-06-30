use axum::{Json, http::StatusCode};
use uuid::Uuid;

use crate::domain::{Job, JobKind, Priority};

pub async fn list_jobs() -> (StatusCode, Json<Vec<Job>>) {
    let jobs = vec![
        Job {
            id: Uuid::new_v4(),
            name: "Name_1".to_string(),
            kind: JobKind::SendEmail {
                to: "tuulwq@gmail.com".to_string(),
                body: "Hello, test body".to_string(),
            },
            priority: Priority::Normal,
            max_retries: 3,
        },
        Job {
            id: Uuid::new_v4(),
            name: "Name_2".to_string(),
            kind: JobKind::SendEmail {
                to: "tuulwq@gmail.com".to_string(),
                body: "Hello, test body".to_string(),
            },
            priority: Priority::Normal,
            max_retries: 3,
        },
    ];

    (StatusCode::OK, Json(jobs))
}
