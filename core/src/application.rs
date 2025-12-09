use crate::repository::PostgresUserRepository;
use crate::services::{KeycloakService, UserServiceImpl};

// Type aliases for concrete implementations
type UserRepo = PostgresUserRepository;
type KeycloakClient = KeycloakService;

/// Application service facade that composes all services.
/// This provides a single entry point for all business logic operations.
#[derive(Clone)]
pub struct ApplicationService {
    pub user_service: UserServiceImpl<UserRepo, KeycloakClient>,
}

impl ApplicationService {
    pub fn new(user_repo: UserRepo, keycloak_service: KeycloakService) -> Self {
        Self {
            user_service: UserServiceImpl::new(user_repo, keycloak_service),
        }
    }
}
