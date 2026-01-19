use crate::models::{KeycloakUserInfo, UpdateUserRequest};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur when interacting with Keycloak.
#[derive(Debug, Error)]
pub enum KeycloakError {
    #[error("Failed to get admin token: {0}")]
    TokenError(String),

    #[error("User not found: {0}")]
    UserNotFound(Uuid),

    #[error("User not found by username: {0}")]
    UserNotFoundByUsername(String),

    #[error("Failed to get user info: {0}")]
    GetUserError(String),

    #[error("Failed to update user: {0}")]
    UpdateUserError(String),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseError(String),
}

/// Trait for Keycloak client operations.
/// This allows mocking Keycloak in tests.
pub trait KeycloakClient: Send + Sync + Clone {
    fn get_user_info(
        &self,
        sub: Uuid,
    ) -> impl Future<Output = Result<KeycloakUserInfo, KeycloakError>> + Send;

    fn get_user_id_by_username(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<Uuid, KeycloakError>> + Send;

    fn update_user_info(
        &self,
        sub: Uuid,
        update_req: &UpdateUserRequest,
    ) -> impl Future<Output = Result<(), KeycloakError>> + Send;
}

#[derive(Debug, Deserialize)]
struct KeycloakTokenResponse {
    access_token: String,
    #[allow(dead_code)]
    expires_in: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct KeycloakUser {
    id: String,
    username: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct KeycloakUserIdOnly {
    id: String,
}

#[derive(Clone)]
pub struct KeycloakService {
    client: Client,
    base_url: String,
    realm: String,
    client_id: String,
    client_secret: String,
}

impl KeycloakService {
    pub fn new(base_url: String, realm: String, client_id: String, client_secret: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            realm,
            client_id,
            client_secret,
        }
    }

    async fn get_admin_token(&self) -> Result<String, KeycloakError> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.base_url, self.realm
        );

        let mut params = HashMap::new();
        params.insert("grant_type", "client_credentials");
        params.insert("client_id", &self.client_id);
        params.insert("client_secret", &self.client_secret);

        let response = self.client.post(&token_url).form(&params).send().await?;

        if !response.status().is_success() {
            return Err(KeycloakError::TokenError(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let token_response: KeycloakTokenResponse = response
            .json()
            .await
            .map_err(|e| KeycloakError::ParseError(e.to_string()))?;
        Ok(token_response.access_token)
    }

    pub async fn get_user_info(&self, sub: Uuid) -> Result<KeycloakUserInfo, KeycloakError> {
        let token = self.get_admin_token().await?;

        let user_url = format!(
            "{}/admin/realms/{}/users/{}",
            self.base_url, self.realm, sub
        );

        let response = self
            .client
            .get(&user_url)
            .bearer_auth(&token)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(KeycloakError::UserNotFound(sub));
        }

        if !response.status().is_success() {
            return Err(KeycloakError::GetUserError(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let keycloak_user: KeycloakUser = response
            .json()
            .await
            .map_err(|e| KeycloakError::ParseError(e.to_string()))?;

        Ok(KeycloakUserInfo {
            username: keycloak_user.username,
            email: keycloak_user.email,
        })
    }

    pub async fn get_user_id_by_username(&self, username: &str) -> Result<Uuid, KeycloakError> {
        let token = self.get_admin_token().await?;

        let users_url = format!(
            "{}/admin/realms/{}/users?username={}&exact=true",
            self.base_url, self.realm, username
        );

        let response = self
            .client
            .get(&users_url)
            .bearer_auth(&token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(KeycloakError::GetUserError(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let users: Vec<KeycloakUserIdOnly> = response
            .json()
            .await
            .map_err(|e| KeycloakError::ParseError(e.to_string()))?;

        let user = users
            .into_iter()
            .next()
            .ok_or_else(|| KeycloakError::UserNotFoundByUsername(username.to_string()))?;

        Uuid::parse_str(&user.id)
            .map_err(|e| KeycloakError::ParseError(format!("Invalid UUID: {}", e)))
    }

    pub async fn update_user_info(
        &self,
        sub: Uuid,
        update_req: &UpdateUserRequest,
    ) -> Result<(), KeycloakError> {
        let token = self.get_admin_token().await?;

        let user_url = format!(
            "{}/admin/realms/{}/users/{}",
            self.base_url, self.realm, sub
        );

        let mut update_data = HashMap::new();
        if let Some(username) = &update_req.username {
            update_data.insert("username", serde_json::json!(username));
        }
        if let Some(email) = &update_req.email {
            update_data.insert("email", serde_json::json!(email));
        }

        let response = self
            .client
            .put(&user_url)
            .bearer_auth(&token)
            .json(&update_data)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(KeycloakError::UserNotFound(sub));
        }

        if !response.status().is_success() {
            return Err(KeycloakError::UpdateUserError(format!(
                "HTTP {}",
                response.status()
            )));
        }

        Ok(())
    }
}

impl KeycloakClient for KeycloakService {
    async fn get_user_info(&self, sub: Uuid) -> Result<KeycloakUserInfo, KeycloakError> {
        KeycloakService::get_user_info(self, sub).await
    }

    async fn get_user_id_by_username(&self, username: &str) -> Result<Uuid, KeycloakError> {
        KeycloakService::get_user_id_by_username(self, username).await
    }

    async fn update_user_info(
        &self,
        sub: Uuid,
        update_req: &UpdateUserRequest,
    ) -> Result<(), KeycloakError> {
        KeycloakService::update_user_info(self, sub, update_req).await
    }
}
