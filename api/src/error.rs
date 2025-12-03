use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::ServiceUnavailable(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg.clone()),
            ApiError::InternalServerError(msg) => {
                tracing::error!("Internal server error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", err);
        ApiError::InternalServerError("Database error".to_string())
    }
}

impl From<user_core::CoreError> for ApiError {
    fn from(err: user_core::CoreError) -> Self {
        match err {
            user_core::CoreError::DatabaseError(e) => {
                tracing::error!("Database error: {}", e);
                ApiError::InternalServerError("Database error".to_string())
            }
            user_core::CoreError::NotFound(msg) => ApiError::NotFound(msg),
            user_core::CoreError::BadRequest(msg) => ApiError::BadRequest(msg),
            user_core::CoreError::Unauthorized(msg) => ApiError::Unauthorized(msg),
            user_core::CoreError::InternalError(msg) => ApiError::InternalServerError(msg),
            user_core::CoreError::KeycloakError(msg) => {
                tracing::error!("Keycloak error: {}", msg);
                ApiError::ServiceUnavailable("Authentication service error".to_string())
            }
        }
    }
}
