use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;
use time::OffsetDateTime;

use crate::{
    domain::{DispatchPolicy, PolicyOption},
    service::dispatch_policy::DispatchPolicyServiceError,
};

#[derive(Serialize)]
pub struct DispatchPolicyResponse {
    id: u64,
    policy: PolicyOption,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
}

impl From<DispatchPolicy> for DispatchPolicyResponse {
    fn from(policy: DispatchPolicy) -> Self {
        Self {
            id: policy.id,
            policy: policy.policy,
            created_at: policy.created_at,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DispatchPolicyError {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Service(#[from] DispatchPolicyServiceError),
}

impl IntoResponse for DispatchPolicyError {
    fn into_response(self) -> Response {
        let (code, message) = match &self {
            DispatchPolicyError::Validation(error) => (StatusCode::BAD_REQUEST, error.to_string()),
            DispatchPolicyError::Service(DispatchPolicyServiceError::NotFound) => (
                StatusCode::NOT_FOUND,
                "Dispatch policy not found".to_string(),
            ),
            DispatchPolicyError::Service(DispatchPolicyServiceError::Storage(_)) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        (code, Json(json!({ "message": message }))).into_response()
    }
}
