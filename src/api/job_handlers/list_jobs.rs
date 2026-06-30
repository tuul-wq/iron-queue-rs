use axum::{Json, http::StatusCode};
use uuid::Uuid;

use crate::domain::Job;

pub async fn list_jobs() -> (StatusCode, Json<Vec<Job>>) {
    let jobs = vec![
        Job {
            id: Uuid::new_v4(),
            name: "Name_1".to_string(),
        },
        Job {
            id: Uuid::new_v4(),
            name: "Name_2".to_string(),
        },
    ];

    (StatusCode::OK, Json(jobs))
}
