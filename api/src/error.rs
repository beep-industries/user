use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use user_core::CoreError;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Wrapper type to implement IntoResponse for CoreError
pub struct ApiError(pub CoreError);

impl From<CoreError> for ApiError {
    fn from(err: CoreError) -> Self {
        ApiError(err)
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError(CoreError::from(err))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self.0 {
            CoreError::DatabaseError(e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            CoreError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            CoreError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            CoreError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            CoreError::InternalError(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            CoreError::KeycloakError(msg) => {
                tracing::error!("Keycloak error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Keycloak error".to_string())
            }
        };

        let body = Json(ErrorResponse {
            error: error_message,
        });

        (status, body).into_response()
    }
}
