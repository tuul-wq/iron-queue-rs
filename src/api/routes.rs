use axum::{
    Router,
    routing::{get, post},
};

use crate::service::jobs::JobService;

use super::health::handlers as health_handlers;
use super::jobs::handlers as job_handlers;

#[derive(Clone)]
pub struct AppState {
    pub job_service: JobService,
}

pub fn setup_routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handlers::health_check))
        .route("/jobs", get(job_handlers::list_jobs))
        .route("/jobs", post(job_handlers::enqueue_job))
        .route("/jobs/{id}", get(job_handlers::get_job))
        .route("/jobs/{id}/cancel", post(job_handlers::cancel_job))
        .with_state(state)
}
