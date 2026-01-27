use serde::Deserialize;
use std::env;
use std::fmt;

#[derive(Debug)]
pub struct ConfigError {
    pub missing_vars: Vec<&'static str>,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Missing required environment variables: {}",
            self.missing_vars.join(", ")
        )
    }
}

impl std::error::Error for ConfigError {}

fn require_env(name: &'static str, missing: &mut Vec<&'static str>) -> Option<String> {
    match env::var(name) {
        Ok(val) => Some(val),
        Err(_) => {
            missing.push(name);
            None
        }
    }
}

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
    pub content_service_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        let mut missing = Vec::new();

        let database_url = require_env("DATABASE_URL", &mut missing);
        let server_host = require_env("SERVER_HOST", &mut missing);
        let server_port = require_env("SERVER_PORT", &mut missing);
        let health_port = require_env("HEALTH_PORT", &mut missing);
        let keycloak_url = require_env("KEYCLOAK_URL", &mut missing);
        let keycloak_internal_url = require_env("KEYCLOAK_INTERNAL_URL", &mut missing);
        let keycloak_realm = require_env("KEYCLOAK_REALM", &mut missing);
        let keycloak_client_id = require_env("KEYCLOAK_CLIENT_ID", &mut missing);
        let keycloak_client_secret: Option<String> =
            require_env("KEYCLOAK_CLIENT_SECRET", &mut missing);

        let content_service_url = require_env("CONTENT_SERVICE_URL", &mut missing);

        if !missing.is_empty() {
            return Err(ConfigError {
                missing_vars: missing,
            });
        }

        Ok(Config {
            database_url: database_url.unwrap(),
            server_host: server_host.unwrap(),
            server_port: server_port
                .unwrap()
                .parse()
                .expect("SERVER_PORT must be a valid u16"),
            health_port: health_port
                .unwrap()
                .parse()
                .expect("HEALTH_PORT must be a valid u16"),
            keycloak_url: keycloak_url.unwrap(),
            keycloak_internal_url: keycloak_internal_url.unwrap(),
            keycloak_realm: keycloak_realm.unwrap(),
            keycloak_client_id: keycloak_client_id.unwrap(),
            keycloak_client_secret: keycloak_client_secret.unwrap(),
            content_service_url: content_service_url.unwrap(),
        })
    }
}
