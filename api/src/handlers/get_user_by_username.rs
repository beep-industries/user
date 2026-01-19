use crate::error::ApiError;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;
use user_core::{UserBasicInfo, UserService};

#[utoipa::path(
    get,
    path = "/users/username/{username}",
    tag = "internal",
    params(
        ("username" = String, Path, description = "Keycloak username")
    ),
    responses(
        (status = 200, description = "User information retrieved successfully", body = UserBasicInfo),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_user_by_username(
    Path(username): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserBasicInfo>, ApiError> {
    let user = state
        .service
        .user_service
        .get_user_by_username(&username)
        .await?;
    Ok(Json(user))
}
