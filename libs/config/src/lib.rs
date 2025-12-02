use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub health_port: u16,
    pub keycloak_url: String,
    pub keycloak_internal_url: String,
    pub keycloak_realm: String,
    pub keycloak_client_id: String,
    pub keycloak_client_secret: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let keycloak_url = env::var("KEYCLOAK_URL")?;
        let keycloak_internal_url = env::var("KEYCLOAK_INTERNAL_URL")
            .unwrap_or_else(|_| keycloak_url.clone());

        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            health_port: env::var("HEALTH_PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()?,
            keycloak_url,
            keycloak_internal_url,
            keycloak_realm: env::var("KEYCLOAK_REALM")?,
            keycloak_client_id: env::var("KEYCLOAK_CLIENT_ID")?,
            keycloak_client_secret: env::var("KEYCLOAK_CLIENT_SECRET")?,
            jwt_secret: env::var("JWT_SECRET")?,
        })
    }
}
