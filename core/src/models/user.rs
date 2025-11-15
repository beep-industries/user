use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub profile_picture: Option<String>,
    pub status: String,
    pub keycloak_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Param {
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
    pub username: String,
    pub profile_picture: Option<String>,
    pub status: String,
    pub keycloak_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakUserInfo {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFullInfo {
    pub id: Uuid,
    pub username: String,
    pub profile_picture: Option<String>,
    pub status: String,
    pub keycloak_id: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub profile_picture: Option<String>,
    pub keycloak_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub profile_picture: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateKeycloakUserRequest {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateParamRequest {
    pub theme: Option<String>,
    pub lang: Option<String>,
}
