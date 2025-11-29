use crate::error::ApiError;
use crate::state::AppState;
use axum::{
    extract::{Extension, State},
    Json,
};
use std::sync::Arc;
use user_core::{UpdateUserRequest, User, UserBasicInfo, UserService};

#[utoipa::path(
    put,
    path = "/users/me",
    tag = "users",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserBasicInfo),
        (status = 400, description = "Bad request - Invalid input"),
        (status = 401, description = "Unauthorized - Invalid or missing JWT token"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_current_user(
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserBasicInfo>, ApiError> {
    let updated_user = state.service.user_service.update_user(&user, req).await?;
    Ok(Json(updated_user))
}
