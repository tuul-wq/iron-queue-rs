use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::service::{dispatch_policy::DispatchPolicyService, jobs::JobService};

use super::dispatch_policy::handlers as policy_handlers;
use super::health::handlers as health_handlers;
use super::jobs::handlers as job_handlers;

#[derive(Clone)]
pub struct AppState {
    pub dispatch_policy_service: DispatchPolicyService,
    pub job_service: JobService,
}

pub fn setup_routes(state: AppState) -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &axum::http::Request<_>| {
            tracing::info_span!(
                "http_request",
                method = %request.method(),
                path = request.uri().path(),
            )
        })
        .on_request(DefaultOnRequest::new().level(Level::DEBUG))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
        .on_failure(DefaultOnFailure::new().level(Level::ERROR));

    Router::new()
        .route("/health", get(health_handlers::health_check))
        .route("/jobs", get(job_handlers::list_jobs))
        .route("/jobs", post(job_handlers::enqueue_job))
        .route("/jobs/{id}", get(job_handlers::get_job))
        .route("/jobs/{id}/cancel", post(job_handlers::cancel_job))
        .route("/dispatch_policy", get(policy_handlers::get_latest_policy))
        .route("/dispatch_policy", post(policy_handlers::add_policy))
        .route(
            "/dispatch_policy/history",
            get(policy_handlers::policy_history),
        )
        .layer(trace_layer)
        .with_state(state)
}
