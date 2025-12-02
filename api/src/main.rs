mod error;
mod handlers;
mod middleware;
mod openapi;
mod state;

use crate::{
    handlers::{
        get_current_user, get_current_user_settings, get_user_by_id, update_current_user,
        update_current_user_settings,
    },
    middleware::auth_middleware,
    openapi::ApiDoc,
    state::AppState,
};
use axum::{middleware as axum_middleware, routing::get, Json, Router};
use beep_auth::KeycloakAuthRepository;
use clap::{Parser, Subcommand};
use config::Config;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use user_core::{ApplicationService, KeycloakService, PostgresUserRepository};

#[derive(Parser)]
#[command(name = "user-service")]
#[command(about = "User service CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the web service
    Run,
    /// Run database migrations
    Migrate,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    let config = Config::from_env()?;

    match cli.command {
        Commands::Migrate => {
            tracing::info!("Connecting to database...");
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&config.database_url)
                .await?;

            tracing::info!("Running migrations...");
            sqlx::migrate!("../migrations").run(&pool).await?;
            tracing::info!("Migrations completed successfully");
        }
        Commands::Run => {
            tracing::info!("Connecting to database...");
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&config.database_url)
                .await?;

            tracing::info!("Initializing services...");
            let user_repo = PostgresUserRepository::new(pool);
            let auth_repository = KeycloakAuthRepository::new(
                format!(
                    "{}/realms/{}",
                    config.keycloak_internal_url, config.keycloak_realm
                ),
                None,
            );
            let keycloak_service = KeycloakService::new(
                config.keycloak_internal_url,
                config.keycloak_realm,
                config.keycloak_client_id,
                config.keycloak_client_secret,
            );
            let service = ApplicationService::new(user_repo, keycloak_service);

            let app_state = Arc::new(AppState::new(service, auth_repository));

            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any);

            let protected_routes = Router::new()
                .route("/users/me", get(get_current_user).put(update_current_user))
                .route(
                    "/users/me/settings",
                    get(get_current_user_settings).put(update_current_user_settings),
                )
                .route("/users/:user_id", get(get_user_by_id))
                .layer(axum_middleware::from_fn_with_state(app_state.clone(), auth_middleware))
                .with_state(app_state);

            // Public routes (no authentication required)
            let public_routes = Router::new()
                .merge(Scalar::with_url("/docs", ApiDoc::openapi()));

            let app = Router::new()
                .merge(public_routes)
                .merge(protected_routes)
                .layer(cors);

            let health_router = Router::new().route(
                "/health",
                get(|| async { Json(serde_json::json!({ "status": "ok" })) }),
            );

            let api_addr = format!("{}:{}", config.server_host, config.server_port);
            let health_addr = format!("{}:{}", config.server_host, config.health_port);

            let api_listener = tokio::net::TcpListener::bind(&api_addr).await?;
            let health_listener = tokio::net::TcpListener::bind(&health_addr).await?;

            tracing::info!("API server listening on {}", api_addr);
            tracing::info!("Health server listening on {}", health_addr);

            tokio::try_join!(
                axum::serve(api_listener, app),
                axum::serve(health_listener, health_router),
            )?;
        }
    }

    Ok(())
}
