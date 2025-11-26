use crate::{error::ApiError, middleware::Claims};
use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use user_core::{
    CoreError, KeycloakService, Setting, UpdateKeycloakUserRequest, UpdateSettingRequest,
    UpdateUserRequest, UserBasicInfo, UserFullInfo, UserRepository,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: UserRepository,
    pub keycloak_service: KeycloakService,
}

#[derive(Deserialize)]
pub struct FullInfoQuery {
    #[serde(default)]
    pub full_info: bool,
}

pub async fn get_current_user(
    Extension(claims): Extension<Claims>,
    Query(query): Query<FullInfoQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let sub = claims.sub;

    let user = state
        .user_repo
        .get_user_by_sub(&sub)
        .await?
        .ok_or_else(|| CoreError::NotFound("User not found".to_string()))?;

    if query.full_info {
        let keycloak_info = state
            .keycloak_service
            .get_user_info(&sub)
            .await
            .map_err(|e| CoreError::KeycloakError(e.to_string()))?;

        let full_info = UserFullInfo {
            id: user.id,
            display_name: user.display_name,
            profile_picture: user.profile_picture,
            status: user.status,
            sub: user.sub,
            username: keycloak_info.username,
            email: keycloak_info.email,
            first_name: keycloak_info.first_name,
            last_name: keycloak_info.last_name,
        };

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

pub async fn get_user_by_id(
    Path(user_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserBasicInfo>, ApiError> {
    let user = state
        .user_repo
        .get_user_by_id(user_id)
        .await?
        .ok_or_else(|| CoreError::NotFound("User not found".to_string()))?;

    let basic_info = UserBasicInfo {
        id: user.id,
        display_name: user.display_name,
        profile_picture: user.profile_picture,
        status: user.status,
        sub: user.sub,
    };

    Ok(Json(basic_info))
}

pub async fn update_current_user(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserBasicInfo>, ApiError> {
    let sub = claims.sub;

    let user = state
        .user_repo
        .get_user_by_sub(&sub)
        .await?
        .ok_or_else(|| CoreError::NotFound("User not found".to_string()))?;

    let updated_user = state.user_repo.update_user(user.id, req).await?;

    let basic_info = UserBasicInfo {
        id: updated_user.id,
        display_name: updated_user.display_name,
        profile_picture: updated_user.profile_picture,
        status: updated_user.status,
        sub: updated_user.sub,
    };

    Ok(Json(basic_info))
}

pub async fn update_current_user_keycloak_info(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdateKeycloakUserRequest>,
) -> Result<StatusCode, ApiError> {
    let sub = claims.sub;

    let user = state
        .user_repo
        .get_user_by_sub(&sub)
        .await?
        .ok_or_else(|| CoreError::NotFound("User not found".to_string()))?;

    state
        .keycloak_service
        .update_user_info(&user.sub, req)
        .await
        .map_err(|e| CoreError::KeycloakError(e.to_string()))?;

    Ok(StatusCode::OK)
}

pub async fn get_current_user_settings(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Setting>, ApiError> {
    let sub = claims.sub;

    let user = state
        .user_repo
        .get_user_by_sub(&sub)
        .await?
        .ok_or_else(|| CoreError::NotFound("User not found".to_string()))?;

    let setting = state
        .user_repo
        .get_setting_by_user_id(user.id)
        .await?
        .ok_or_else(|| CoreError::NotFound("Setting not found".to_string()))?;

    Ok(Json(setting))
}

pub async fn update_current_user_settings(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdateSettingRequest>,
) -> Result<Json<Setting>, ApiError> {
    let sub = claims.sub;

    let user = state
        .user_repo
        .get_user_by_sub(&sub)
        .await?
        .ok_or_else(|| CoreError::NotFound("User not found".to_string()))?;

    let setting = state.user_repo.update_setting(user.id, req).await?;

    Ok(Json(setting))
}
