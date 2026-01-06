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
    path = "/users/display_name/{display_name}",
    tag = "users",
    params(
        ("display_name" = String, Path, description = "User display name")
    ),
    responses(
        (status = 200, description = "User information retrieved successfully", body = UserBasicInfo),
        (status = 401, description = "Unauthorized - Invalid or missing JWT token"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user_by_display_name(
    Path(display_name): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserBasicInfo>, ApiError> {
    let user = state
        .service
        .user_service
        .get_user_by_display_name(&display_name)
        .await?;
    Ok(Json(user))
}
