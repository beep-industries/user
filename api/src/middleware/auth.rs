use crate::state::AppState;
use axum::{
    body::Body,
    extract::{Request, State},
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use beep_auth::{AuthRepository, Identity};
use std::sync::Arc;
use user_core::UserService;
use uuid::Uuid;

fn extract_token_from_bearer(auth_header: &str) -> Option<&str> {
    auth_header.strip_prefix("Bearer ")
}

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = extract_token_from_bearer(auth_header).ok_or(StatusCode::UNAUTHORIZED)?;

    let identity = state.auth_repository.identify(token).await.map_err(|e| {
        tracing::error!("Authentication failed: {:?}", e);
        StatusCode::UNAUTHORIZED
    })?;

    let sub_str = match &identity {
        Identity::User(user) => &user.id,
        Identity::Client(client) => &client.id,
    };

    let sub = Uuid::parse_str(sub_str).map_err(|e| {
        tracing::error!("Invalid sub UUID: {}", e);
        StatusCode::UNAUTHORIZED
    })?;

    // Auto-create user if not exists (first connection after Keycloak registration)
    // TODO: This is not ideal - we're making a DB call on every request.
    // This should be refactored to use a cache or session-based approach to avoid
    // the performance overhead of checking user existence on each authenticated request.
    // For now, we accept this trade-off for simplicity.
    let user = state
        .service
        .user_service
        .get_or_create_user(sub)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get or create user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    req.extensions_mut().insert(identity);
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
