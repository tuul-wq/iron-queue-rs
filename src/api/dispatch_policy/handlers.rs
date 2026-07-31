use axum::{Json, extract::State, http::StatusCode};
use tracing::instrument;

use super::{
    request::UpdateDispatchPolicyRequest,
    response::{DispatchPolicyError, DispatchPolicyResponse},
};

use crate::{api::routes::AppState, domain::UpdateDispatchPolicyCommand};

#[instrument(skip(state, body))]
pub async fn add_policy(
    State(state): State<AppState>,
    Json(body): Json<UpdateDispatchPolicyRequest>,
) -> Result<(StatusCode, Json<DispatchPolicyResponse>), DispatchPolicyError> {
    let command = UpdateDispatchPolicyCommand::try_from(body).map_err(|error| {
        tracing::warn!(error = %error, "dispatch policy request rejected");
        error
    })?;

    let policy = state.dispatch_policy_service.add_policy(command).await?;
    tracing::info!(policy_id = policy.id, "dispatch policy created");

    Ok((StatusCode::CREATED, Json(policy.into())))
}

#[instrument(skip(state))]
pub async fn get_latest_policy(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<DispatchPolicyResponse>), DispatchPolicyError> {
    let policy = state.dispatch_policy_service.get_latest_policy().await?;

    Ok((StatusCode::OK, Json(policy.into())))
}

#[instrument(skip(state))]
pub async fn policy_history(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<DispatchPolicyResponse>>), DispatchPolicyError> {
    let policies = state.dispatch_policy_service.policy_history().await?;

    Ok((
        StatusCode::OK,
        Json(policies.into_iter().map(|policy| policy.into()).collect()),
    ))
}
