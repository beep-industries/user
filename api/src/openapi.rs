use crate::handlers::{GetUsersBySubsRequest, GetUsersBySubsResponse};
use user_core::{Setting, UpdateSettingRequest, UpdateUserRequest, UserBasicInfo, UserFullInfo};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "User Service API",
        version = "1.0.0",
        description = r#"Rust microservice for user management with Keycloak integration.

## Data Storage
- **Keycloak Database**: Stores authentication data (username, email)
- **User Service Database**: Stores application-specific data (display_name, profile_picture, description, settings)"#,
        contact(
            name = "API Support",
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development server")
    ),
    paths(
        crate::handlers::get_current_user,
        crate::handlers::update_current_user,
        crate::handlers::get_current_user_settings,
        crate::handlers::update_current_user_settings,
        crate::handlers::get_user_by_sub,
        crate::handlers::get_users_by_subs,
    ),
    components(
        schemas(
            UserBasicInfo,
            UserFullInfo,
            UpdateUserRequest,
            Setting,
            UpdateSettingRequest,
            GetUsersBySubsRequest,
            GetUsersBySubsResponse
        )
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "settings", description = "User settings endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("JWT Bearer token from Keycloak"))
                        .build(),
                ),
            )
        }
    }
}
