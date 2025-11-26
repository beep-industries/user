use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub status: String,
    pub sub: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Setting {
    pub id: Uuid,
    pub user_id: Uuid,
    pub theme: Option<String>,
    pub lang: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBasicInfo {
    pub id: Uuid,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub status: String,
    pub sub: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakUserInfo {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFullInfo {
    pub id: Uuid,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub status: String,
    pub sub: String,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub sub: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    // Local DB fields
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub status: Option<String>,
    // Keycloak fields
    pub username: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl UpdateUserRequest {
    pub fn has_local_fields(&self) -> bool {
        self.display_name.is_some() || self.profile_picture.is_some() || self.status.is_some()
    }

    pub fn has_keycloak_fields(&self) -> bool {
        self.username.is_some() || self.email.is_some() || self.first_name.is_some() || self.last_name.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSettingRequest {
    pub theme: Option<String>,
    pub lang: Option<String>,
}
