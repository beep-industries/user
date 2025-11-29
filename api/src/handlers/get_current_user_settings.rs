use crate::error::ApiError;
use crate::state::AppState;
use axum::{
    extract::{Extension, State},
    Json,
};
use std::sync::Arc;
use user_core::{Setting, User, UserService};

#[utoipa::path(
    get,
    path = "/users/me/settings",
    tag = "settings",
    responses(
        (status = 200, description = "User settings retrieved successfully", body = Setting),
        (status = 401, description = "Unauthorized - Invalid or missing JWT token"),
        (status = 404, description = "Settings not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_current_user_settings(
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Setting>, ApiError> {
    let setting = state.service.user_service.get_user_settings(user.id).await?;
    Ok(Json(setting))
}
