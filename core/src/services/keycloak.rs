use crate::models::{KeycloakUserInfo, UpdateKeycloakUserRequest};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
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

    async fn get_admin_token(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.base_url, self.realm
        );

        let mut params = HashMap::new();
        params.insert("grant_type", "client_credentials");
        params.insert("client_id", &self.client_id);
        params.insert("client_secret", &self.client_secret);

        let response = self
            .client
            .post(&token_url)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to get admin token: {}", response.status()).into());
        }

        let token_response: KeycloakTokenResponse = response.json().await?;
        Ok(token_response.access_token)
    }

    pub async fn get_user_info(
        &self,
        keycloak_id: &str,
    ) -> Result<KeycloakUserInfo, Box<dyn std::error::Error + Send + Sync>> {
        let token = self.get_admin_token().await?;

        let user_url = format!(
            "{}/admin/realms/{}/users/{}",
            self.base_url, self.realm, keycloak_id
        );

        let response = self
            .client
            .get(&user_url)
            .bearer_auth(&token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to get user info: {}", response.status()).into());
        }

        let keycloak_user: KeycloakUser = response.json().await?;

        Ok(KeycloakUserInfo {
            username: keycloak_user.username,
            email: keycloak_user.email,
            first_name: keycloak_user.first_name,
            last_name: keycloak_user.last_name,
        })
    }

    pub async fn update_user_info(
        &self,
        keycloak_id: &str,
        update_req: UpdateKeycloakUserRequest,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let token = self.get_admin_token().await?;

        let user_url = format!(
            "{}/admin/realms/{}/users/{}",
            self.base_url, self.realm, keycloak_id
        );

        let mut update_data = HashMap::new();
        if let Some(username) = update_req.username {
            update_data.insert("username", serde_json::json!(username));
        }
        if let Some(email) = update_req.email {
            update_data.insert("email", serde_json::json!(email));
        }
        if let Some(first_name) = update_req.first_name {
            update_data.insert("firstName", serde_json::json!(first_name));
        }
        if let Some(last_name) = update_req.last_name {
            update_data.insert("lastName", serde_json::json!(last_name));
        }

        let response = self
            .client
            .put(&user_url)
            .bearer_auth(&token)
            .json(&update_data)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to update user info: {}", response.status()).into());
        }

        Ok(())
    }
}
