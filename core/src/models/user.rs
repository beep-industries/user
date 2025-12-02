use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct User {
    pub id: Uuid,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub description: Option<String>,
    pub sub: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct Setting {
    pub id: Uuid,
    pub user_id: Uuid,
    pub theme: Option<String>,
    pub lang: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct UserBasicInfo {
    pub id: Uuid,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub description: Option<String>,
    pub sub: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct KeycloakUserInfo {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct UserFullInfo {
    pub id: Uuid,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub description: Option<String>,
    pub sub: String,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct CreateUserRequest {
    pub sub: String,
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
    /// First name (stored in Keycloak Database)
    pub first_name: Option<String>,
    /// Last name (stored in Keycloak Database)
    pub last_name: Option<String>,
}

impl UpdateUserRequest {
    pub fn has_local_fields(&self) -> bool {
        self.display_name.is_some() || self.profile_picture.is_some() || self.description.is_some()
    }

    pub fn has_keycloak_fields(&self) -> bool {
        self.username.is_some() || self.email.is_some() || self.first_name.is_some() || self.last_name.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct UpdateSettingRequest {
    /// Theme preference (e.g., "dark", "light")
    pub theme: Option<String>,
    /// Language preference (e.g., "en", "fr")
    pub lang: Option<String>,
}
