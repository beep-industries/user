use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Keycloak error: {0}")]
    KeycloakError(String),
}

impl From<Box<dyn std::error::Error + Send + Sync>> for CoreError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        CoreError::InternalError(err.to_string())
    }
}
