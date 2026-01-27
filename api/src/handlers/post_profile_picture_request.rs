use std::sync::Arc;

use axum::{Extension, extract::State};
use user_core::{User, UserService};

use crate::{error::ApiError, state::AppState};

#[utoipa::path(
    post,
    path = "/users/me/profile-picture",
    responses(
        (status = 200, description = "Profile picture updated successfully"),
        (status = 401, description = "Unauthorized - Invalid or missing JWT token"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn post_profile_picture_request(
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState>>,
    ) -> Result<String, ApiError> {
    let url = state.service.user_service.generate_profile_picture_url(&user).await?;
    Ok(url)
}
