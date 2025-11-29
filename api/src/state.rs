use crate::middleware::JwksCache;
use std::sync::Arc;
use user_core::ApplicationService;

/// Application state shared across all handlers.
/// Uses Arc for cheap cloning in async context.
#[derive(Clone)]
pub struct AppState {
    pub service: ApplicationService,
    pub jwks_cache: Arc<JwksCache>,
}

impl AppState {
    pub fn new(service: ApplicationService, jwks_cache: JwksCache) -> Self {
        Self {
            service,
            jwks_cache: Arc::new(jwks_cache),
        }
    }
}
