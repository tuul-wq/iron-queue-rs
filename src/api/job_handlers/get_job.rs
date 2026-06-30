use axum::{Json, extract::Path, http::StatusCode};
use uuid::Uuid;

use crate::domain::{Job, JobKind, Priority};

pub async fn get_job(Path(job_id): Path<Uuid>) -> (StatusCode, Json<Job>) {
    let job = Job {
        id: Uuid::new_v4(),
        name: "Name_1".to_string(),
        kind: JobKind::SendEmail {
            to: "tuulwq@gmail.com".to_string(),
            body: "Hello, test body".to_string(),
        },
        priority: Priority::Normal,
        max_retries: 3,
    };

    (StatusCode::OK, Json(job))
}
