use crate::error::ApiError;
use crate::state::AppState;
use axum::{
    extract::{Extension, Query, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::IntoParams;
use user_core::{User, UserBasicInfo, UserFullInfo, UserService};

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
