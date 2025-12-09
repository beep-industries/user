use crate::services::KeycloakError;
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
    KeycloakError(#[from] KeycloakError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_error_not_found_displays_correctly() {
        let err = CoreError::NotFound("User not found".to_string());
        assert_eq!(err.to_string(), "Not found: User not found");
    }

    #[test]
    fn core_error_bad_request_displays_correctly() {
        let err = CoreError::BadRequest("Invalid input".to_string());
        assert_eq!(err.to_string(), "Bad request: Invalid input");
    }

    #[test]
    fn core_error_unauthorized_displays_correctly() {
        let err = CoreError::Unauthorized("Invalid token".to_string());
        assert_eq!(err.to_string(), "Unauthorized: Invalid token");
    }

    #[test]
    fn core_error_internal_error_displays_correctly() {
        let err = CoreError::InternalError("Something went wrong".to_string());
        assert_eq!(err.to_string(), "Internal error: Something went wrong");
    }

    #[test]
    fn core_error_keycloak_error_displays_correctly() {
        let keycloak_err = KeycloakError::TokenError("Connection failed".to_string());
        let err = CoreError::KeycloakError(keycloak_err);
        assert_eq!(
            err.to_string(),
            "Keycloak error: Failed to get admin token: Connection failed"
        );
    }

    #[test]
    fn core_error_from_keycloak_error() {
        let keycloak_err = KeycloakError::UserNotFound(uuid::Uuid::nil());
        let core_err: CoreError = keycloak_err.into();

        assert!(matches!(core_err, CoreError::KeycloakError(_)));
    }
}
