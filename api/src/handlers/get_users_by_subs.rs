use crate::error::ApiError;
use crate::state::AppState;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use user_core::{UserBasicInfo, UserService};
use uuid::Uuid;

/// Maximum number of subs that can be requested at once
const MAX_SUBS_PER_REQUEST: usize = 100;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct GetUsersBySubsRequest {
    /// List of user sub IDs to fetch (max 100)
    pub subs: Vec<Uuid>,
    /// Number of results to skip (for pagination)
    #[serde(default)]
    pub offset: usize,
    /// Maximum number of results to return (default: 20, max: 100)
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    20
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct GetUsersBySubsResponse {
    /// List of users found
    pub users: Vec<UserBasicInfo>,
    /// Total number of users found (before pagination)
    pub total: usize,
    /// Current offset
    pub offset: usize,
    /// Current limit
    pub limit: usize,
}

#[utoipa::path(
    post,
    path = "/users/bart",
    tag = "users",
    request_body = GetUsersBySubsRequest,
    responses(
        (status = 200, description = "Users information retrieved successfully", body = GetUsersBySubsResponse),
        (status = 400, description = "Bad request - Too many subs requested"),
        (status = 401, description = "Unauthorized - Invalid or missing JWT token"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_users_by_subs(
    State(state): State<Arc<AppState>>,
    Json(request): Json<GetUsersBySubsRequest>,
) -> Result<Json<GetUsersBySubsResponse>, ApiError> {
    // Validate request
    if request.subs.len() > MAX_SUBS_PER_REQUEST {
        return Err(ApiError::BadRequest(format!(
            "Too many subs requested. Maximum is {}",
            MAX_SUBS_PER_REQUEST
        )));
    }

    let limit = request.limit.min(MAX_SUBS_PER_REQUEST);

    // Fetch all matching users
    let all_users = state
        .service
        .user_service
        .get_users_by_subs(&request.subs)
        .await?;

    let total = all_users.len();

    // Apply pagination
    let users: Vec<UserBasicInfo> = all_users
        .into_iter()
        .skip(request.offset)
        .take(limit)
        .collect();

    Ok(Json(GetUsersBySubsResponse {
        users,
        total,
        offset: request.offset,
        limit,
    }))
}
