use axum::{
    Router,
    routing::{get, post},
};

use super::general_handlers;
use super::job_handlers;

pub fn setup_routes() -> Router {
    Router::new()
        .route("/health", get(general_handlers::health_check))
        .route("/jobs", get(job_handlers::list_jobs))
        .route("/jobs", post(job_handlers::create_job))
        .route("/jobs/{id}", get(job_handlers::get_job))
        .route("/jobs/{id}/cancel", post(job_handlers::cancel_job))
}
