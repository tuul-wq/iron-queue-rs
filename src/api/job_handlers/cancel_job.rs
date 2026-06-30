use axum::{extract::Path, http::StatusCode};
use uuid::Uuid;

pub async fn cancel_job(Path(job_id): Path<Uuid>) -> StatusCode {
    StatusCode::OK
}
