use crate::handlers::AppState;
use axum::{
    body::Body,
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = match auth_header {
        Some(header) => header,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &validation,
    )
    .map_err(|e| {
        tracing::error!("JWT validation error: {}", e);
        StatusCode::UNAUTHORIZED
    })?;

    // Auto-create user if not exists (first connection after Keycloak registration)
    let user = state
        .user_repo
        .get_or_create_user(&token_data.claims.sub)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get or create user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    req.extensions_mut().insert(token_data.claims);
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
