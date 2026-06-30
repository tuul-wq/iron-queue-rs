use axum::{Json, extract::Path, http::StatusCode};
use uuid::Uuid;

use crate::domain::Job;

pub async fn get_job(Path(job_id): Path<Uuid>) -> (StatusCode, Json<Job>) {
    let job = Job {
        id: Uuid::new_v4(),
        name: "Name_1".to_string(),
    };

    (StatusCode::OK, Json(job))
}
