use crate::error::ApiError;
use crate::state::AppState;
use axum::{extract::{Query, State}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use user_core::{UserBasicInfo, UserService};

/// Maximum number of users that can be requested at once
const MAX_LIMIT: i64 = 100;
const DEFAULT_LIMIT: i64 = 20;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct ListUsersQuery {
    /// Number of results to skip (for pagination)
    #[serde(default)]
    pub offset: i64,
    /// Maximum number of results to return (default: 20, max: 100)
    #[serde(default = "default_limit")]
    pub limit: i64,
    /// Search term to filter users by display name
    pub search: Option<String>,
}

fn default_limit() -> i64 {
    DEFAULT_LIMIT
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ListUsersResponse {
    /// List of users found
    pub users: Vec<UserBasicInfo>,
    /// Total number of users (before pagination)
    pub total: i64,
    /// Current offset
    pub offset: i64,
    /// Current limit
    pub limit: i64,
}

#[utoipa::path(
    get,
    path = "/users",
    tag = "admin",
    params(
        ("offset" = i64, Query, description = "Number of results to skip"),
        ("limit" = i64, Query, description = "Maximum number of results to return (default: 20, max: 100)"),
        ("search" = Option<String>, Query, description = "Search term to filter users by display name")
    ),
    responses(
        (status = 200, description = "Users list retrieved successfully", body = ListUsersResponse),
        (status = 400, description = "Bad request - Invalid parameters"),
        (status = 401, description = "Unauthorized - Invalid or missing JWT token"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_users(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<ListUsersResponse>, ApiError> {
    // Validate and cap limit
    let limit = query.limit.min(MAX_LIMIT).max(1);
    let offset = query.offset.max(0);

    // Fetch users with pagination
    let (users, total) = state
        .service
        .user_service
        .list_users(offset, limit, query.search.as_deref())
        .await?;

    Ok(Json(ListUsersResponse {
        users,
        total,
        offset,
        limit,
    }))
}
