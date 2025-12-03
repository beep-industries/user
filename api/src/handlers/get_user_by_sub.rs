use crate::error::ApiError;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use user_core::{UserBasicInfo, UserService};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/users/{sub}",
    tag = "users",
    params(
        ("sub" = Uuid, Path, description = "User sub (UUID)")
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
pub async fn get_user_by_sub(
    Path(sub): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserBasicInfo>, ApiError> {
    let user = state.service.user_service.get_user_by_sub(sub).await?;
    Ok(Json(user))
}
