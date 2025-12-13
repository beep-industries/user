use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct User {
    pub sub: Uuid,
    pub display_name: String,
    pub profile_picture: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct Setting {
    pub sub: Uuid,
    pub theme: Option<String>,
    pub lang: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct UserBasicInfo {
    pub sub: Uuid,
    pub display_name: String,
    pub profile_picture: String,
    pub description: String,
}

impl From<User> for UserBasicInfo {
    fn from(user: User) -> Self {
        Self {
            sub: user.sub,
            display_name: user.display_name,
            profile_picture: user.profile_picture,
            description: user.description,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct KeycloakUserInfo {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct UserFullInfo {
    pub sub: Uuid,
    pub display_name: String,
    pub profile_picture: String,
    pub description: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct CreateUserRequest {
    pub sub: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct UpdateUserRequest {
    /// Display name (stored in User Service Database)
    pub display_name: Option<String>,
    /// Profile picture URL (stored in User Service Database)
    pub profile_picture: Option<String>,
    /// User description (stored in User Service Database)
    pub description: Option<String>,
    /// Username (stored in Keycloak Database)
    pub username: Option<String>,
    /// Email address (stored in Keycloak Database)
    pub email: Option<String>,
}

impl UpdateUserRequest {
    pub fn has_local_fields(&self) -> bool {
        self.display_name.is_some() || self.profile_picture.is_some() || self.description.is_some()
    }

    pub fn has_keycloak_fields(&self) -> bool {
        self.username.is_some() || self.email.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct UpdateSettingRequest {
    /// Theme preference (e.g., "dark", "light")
    pub theme: Option<String>,
    /// Language preference (e.g., "en", "fr")
    pub lang: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod update_user_request {
        use super::*;

        #[test]
        fn has_local_fields_returns_true_when_display_name_is_set() {
            let req = UpdateUserRequest {
                display_name: Some("John".to_string()),
                profile_picture: None,
                description: None,
                username: None,
                email: None,
            };
            assert!(req.has_local_fields());
        }

        #[test]
        fn has_local_fields_returns_true_when_profile_picture_is_set() {
            let req = UpdateUserRequest {
                display_name: None,
                profile_picture: Some("https://example.com/pic.jpg".to_string()),
                description: None,
                username: None,
                email: None,
            };
            assert!(req.has_local_fields());
        }

        #[test]
        fn has_local_fields_returns_true_when_description_is_set() {
            let req = UpdateUserRequest {
                display_name: None,
                profile_picture: None,
                description: Some("A description".to_string()),
                username: None,
                email: None,
            };
            assert!(req.has_local_fields());
        }

        #[test]
        fn has_local_fields_returns_false_when_only_keycloak_fields_are_set() {
            let req = UpdateUserRequest {
                display_name: None,
                profile_picture: None,
                description: None,
                username: Some("john_doe".to_string()),
                email: Some("john@example.com".to_string()),
            };
            assert!(!req.has_local_fields());
        }

        #[test]
        fn has_local_fields_returns_false_when_no_fields_are_set() {
            let req = UpdateUserRequest {
                display_name: None,
                profile_picture: None,
                description: None,
                username: None,
                email: None,
            };
            assert!(!req.has_local_fields());
        }

        #[test]
        fn has_keycloak_fields_returns_true_when_username_is_set() {
            let req = UpdateUserRequest {
                display_name: None,
                profile_picture: None,
                description: None,
                username: Some("john_doe".to_string()),
                email: None,
            };
            assert!(req.has_keycloak_fields());
        }

        #[test]
        fn has_keycloak_fields_returns_true_when_email_is_set() {
            let req = UpdateUserRequest {
                display_name: None,
                profile_picture: None,
                description: None,
                username: None,
                email: Some("john@example.com".to_string()),
            };
            assert!(req.has_keycloak_fields());
        }

        #[test]
        fn has_keycloak_fields_returns_false_when_only_local_fields_are_set() {
            let req = UpdateUserRequest {
                display_name: Some("John".to_string()),
                profile_picture: Some("https://example.com/pic.jpg".to_string()),
                description: Some("A description".to_string()),
                username: None,
                email: None,
            };
            assert!(!req.has_keycloak_fields());
        }

        #[test]
        fn has_keycloak_fields_returns_false_when_no_fields_are_set() {
            let req = UpdateUserRequest {
                display_name: None,
                profile_picture: None,
                description: None,
                username: None,
                email: None,
            };
            assert!(!req.has_keycloak_fields());
        }

        #[test]
        fn has_both_local_and_keycloak_fields() {
            let req = UpdateUserRequest {
                display_name: Some("John".to_string()),
                profile_picture: None,
                description: None,
                username: Some("john_doe".to_string()),
                email: None,
            };
            assert!(req.has_local_fields());
            assert!(req.has_keycloak_fields());
        }
    }

    mod serialization {
        use super::*;

        #[test]
        fn user_basic_info_serializes_correctly() {
            let sub = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
            let info = UserBasicInfo {
                sub,
                display_name: "John Doe".to_string(),
                profile_picture: "https://example.com/pic.jpg".to_string(),
                description: "A developer".to_string(),
            };

            let json = serde_json::to_string(&info).unwrap();
            let parsed: UserBasicInfo = serde_json::from_str(&json).unwrap();

            assert_eq!(parsed.sub, sub);
            assert_eq!(parsed.display_name, "John Doe");
            assert_eq!(parsed.profile_picture, "https://example.com/pic.jpg");
            assert_eq!(parsed.description, "A developer");
        }

        #[test]
        fn user_full_info_serializes_correctly() {
            let sub = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
            let info = UserFullInfo {
                sub,
                display_name: "John Doe".to_string(),
                profile_picture: "https://example.com/pic.jpg".to_string(),
                description: "A developer".to_string(),
                username: "john_doe".to_string(),
                email: "john@example.com".to_string(),
            };

            let json = serde_json::to_string(&info).unwrap();
            let parsed: UserFullInfo = serde_json::from_str(&json).unwrap();

            assert_eq!(parsed.sub, sub);
            assert_eq!(parsed.username, "john_doe");
            assert_eq!(parsed.email, "john@example.com");
        }

        #[test]
        fn update_user_request_deserializes_partial_json() {
            let json = r#"{"display_name": "New Name"}"#;
            let req: UpdateUserRequest = serde_json::from_str(json).unwrap();

            assert_eq!(req.display_name, Some("New Name".to_string()));
            assert!(req.profile_picture.is_none());
            assert!(req.description.is_none());
            assert!(req.username.is_none());
            assert!(req.email.is_none());
        }

        #[test]
        fn update_setting_request_deserializes_partial_json() {
            let json = r#"{"theme": "dark"}"#;
            let req: UpdateSettingRequest = serde_json::from_str(json).unwrap();

            assert_eq!(req.theme, Some("dark".to_string()));
            assert!(req.lang.is_none());
        }

        #[test]
        fn keycloak_user_info_serializes_correctly() {
            let info = KeycloakUserInfo {
                username: "john_doe".to_string(),
                email: "john@example.com".to_string(),
            };

            let json = serde_json::to_string(&info).unwrap();
            let parsed: KeycloakUserInfo = serde_json::from_str(&json).unwrap();

            assert_eq!(parsed.username, "john_doe");
            assert_eq!(parsed.email, "john@example.com");
        }
    }
}
