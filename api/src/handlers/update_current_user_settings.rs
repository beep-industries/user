use crate::error::ApiError;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Extension, State},
};
use std::sync::Arc;
use user_core::{Setting, UpdateSettingRequest, User, UserService};

#[utoipa::path(
    put,
    path = "/users/me/settings",
    tag = "settings",
    request_body = UpdateSettingRequest,
    responses(
        (status = 200, description = "User settings updated successfully", body = Setting),
        (status = 400, description = "Bad request - Invalid input"),
        (status = 401, description = "Unauthorized - Invalid or missing JWT token"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_current_user_settings(
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdateSettingRequest>,
) -> Result<Json<Setting>, ApiError> {
    let setting = state
        .service
        .user_service
        .update_user_settings(user.sub, req)
        .await?;
    Ok(Json(setting))
}
