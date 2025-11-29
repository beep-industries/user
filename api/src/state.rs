use beep_auth::KeycloakAuthRepository;
use std::sync::Arc;
use user_core::ApplicationService;

/// Application state shared across all handlers.
/// Uses Arc for cheap cloning in async context.
#[derive(Clone)]
pub struct AppState {
    pub service: ApplicationService,
    pub auth_repository: Arc<KeycloakAuthRepository>,
}

impl AppState {
    pub fn new(service: ApplicationService, auth_repository: KeycloakAuthRepository) -> Self {
        Self {
            service,
            auth_repository: Arc::new(auth_repository),
        }
    }
}
