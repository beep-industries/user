use crate::error::ApiError;
use crate::state::AppState;
use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::IntoParams;
use user_core::{
    Setting, UpdateSettingRequest, UpdateUserRequest, User, UserBasicInfo, UserFullInfo,
    UserService,
};
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
pub struct FullInfoQuery {
    /// If true, includes Keycloak data (username, email, first name, last name)
    #[serde(default)]
    pub full_info: bool,
}

#[utoipa::path(
    get,
    path = "/users/me",
    tag = "users",
    params(FullInfoQuery),
    responses(
        (status = 200, description = "User information retrieved successfully", body = UserBasicInfo),
        (status = 401, description = "Unauthorized - Invalid or missing JWT token"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_current_user(
    Extension(user): Extension<User>,
    Query(query): Query<FullInfoQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if query.full_info {
        let full_info: UserFullInfo = state.service.user_service.get_user_full_info(&user).await?;
        Ok(Json(serde_json::to_value(full_info).unwrap()))
    } else {
        let basic_info = UserBasicInfo {
            id: user.id,
            display_name: user.display_name,
            profile_picture: user.profile_picture,
            status: user.status,
            sub: user.sub,
        };
        Ok(Json(serde_json::to_value(basic_info).unwrap()))
    }
}

#[utoipa::path(
    get,
    path = "/users/{user_id}",
    tag = "users",
    params(
        ("user_id" = Uuid, Path, description = "User UUID")
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
pub async fn get_user_by_id(
    Path(user_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserBasicInfo>, ApiError> {
    let user = state.service.user_service.get_user_by_id(user_id).await?;
    Ok(Json(user))
}

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
        .update_user_settings(user.id, req)
        .await?;
    Ok(Json(setting))
}
