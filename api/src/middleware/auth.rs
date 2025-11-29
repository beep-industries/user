use crate::state::AppState;
use axum::{
    body::Body,
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use user_core::UserService;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize, Clone)]
struct Jwk {
    kid: String,
    kty: String,
    alg: Option<String>,
    n: Option<String>,
    e: Option<String>,
    #[serde(rename = "use")]
    key_use: Option<String>,
}

#[derive(Clone)]
pub struct JwksCache {
    keys: Arc<RwLock<HashMap<String, DecodingKey>>>,
    keycloak_url: String,
    realm: String,
}

impl JwksCache {
    pub fn new(keycloak_url: &str, realm: &str) -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
            keycloak_url: keycloak_url.to_string(),
            realm: realm.to_string(),
        }
    }

    async fn fetch_jwks(&self) -> Result<Vec<Jwk>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}/realms/{}/protocol/openid-connect/certs",
            self.keycloak_url, self.realm
        );

        let response = reqwest::get(&url).await?;
        let jwks: JwksResponse = response.json().await?;
        Ok(jwks.keys)
    }

    pub async fn get_key(&self, kid: &str) -> Result<DecodingKey, Box<dyn std::error::Error + Send + Sync>> {
        // Check cache first
        {
            let cache = self.keys.read().await;
            if let Some(key) = cache.get(kid) {
                return Ok(key.clone());
            }
        }

        // Fetch JWKS and update cache
        let jwks = self.fetch_jwks().await?;
        let mut cache = self.keys.write().await;

        for jwk in jwks {
            if jwk.kty == "RSA" {
                if let (Some(n), Some(e)) = (&jwk.n, &jwk.e) {
                    if let Ok(key) = DecodingKey::from_rsa_components(n, e) {
                        cache.insert(jwk.kid.clone(), key);
                    }
                }
            }
        }

        cache.get(kid).cloned().ok_or_else(|| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Key with kid {} not found", kid),
            )) as Box<dyn std::error::Error + Send + Sync>
        })
    }
}

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = match auth_header {
        Some(header) => header,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Decode header to get kid
    let header = decode_header(token).map_err(|e| {
        tracing::error!("Failed to decode JWT header: {}", e);
        StatusCode::UNAUTHORIZED
    })?;

    let kid = header.kid.ok_or_else(|| {
        tracing::error!("JWT missing kid in header");
        StatusCode::UNAUTHORIZED
    })?;

    // Get the decoding key from JWKS cache
    let decoding_key = state.jwks_cache.get_key(&kid).await.map_err(|e| {
        tracing::error!("Failed to get JWKS key: {}", e);
        StatusCode::UNAUTHORIZED
    })?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;

    let token_data = decode::<Claims>(token, &decoding_key, &validation).map_err(|e| {
        tracing::error!("JWT validation error: {}", e);
        StatusCode::UNAUTHORIZED
    })?;

    // Auto-create user if not exists (first connection after Keycloak registration)
    // TODO: This is not ideal - we're making a DB call on every request.
    // This should be refactored to use a cache or session-based approach to avoid
    // the performance overhead of checking user existence on each authenticated request.
    // For now, we accept this trade-off for simplicity.
    let user = state
        .service
        .user_service
        .get_or_create_user(&token_data.claims.sub)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get or create user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    req.extensions_mut().insert(token_data.claims);
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
