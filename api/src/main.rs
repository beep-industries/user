mod error;
mod handlers;
mod middleware;

use crate::{
    handlers::{
        get_current_user, get_current_user_params, get_user_by_id, update_current_user,
        update_current_user_keycloak_info, update_current_user_params, AppState,
    },
    middleware::auth_middleware,
};
use axum::{
    middleware as axum_middleware,
    routing::{get, put},
    Router,
};
use clap::{Parser, Subcommand};
use config::Config;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use user_core::{KeycloakService, UserRepository};

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
    env_logger::init();

    let cli = Cli::parse();
    let config = Config::from_env()?;

    match cli.command {
        Commands::Migrate => {
            log::info!("Connecting to database...");
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&config.database_url)
                .await?;

            log::info!("Running migrations...");
            sqlx::migrate!("../migrations").run(&pool).await?;
            log::info!("Migrations completed successfully");
        }
        Commands::Run => {
            log::info!("Connecting to database...");
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&config.database_url)
                .await?;

            log::info!("Initializing services...");
            let user_repo = UserRepository::new(pool);
            let keycloak_service = KeycloakService::new(
                config.keycloak_url,
                config.keycloak_realm,
                config.keycloak_client_id,
                config.keycloak_client_secret,
            );

            let app_state = Arc::new(AppState {
                user_repo,
                keycloak_service,
            });

            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any);

            let protected_routes = Router::new()
                .route("/users/me", get(get_current_user).put(update_current_user))
                .route("/users/me/keycloak", put(update_current_user_keycloak_info))
                .route(
                    "/users/me/params",
                    get(get_current_user_params).put(update_current_user_params),
                )
                .route("/users/:user_id", get(get_user_by_id))
                .layer(axum_middleware::from_fn(auth_middleware))
                .with_state(app_state);

            let app = Router::new().merge(protected_routes).layer(cors);

            let addr = format!("{}:{}", config.server_host, config.server_port);
            log::info!("Server listening on {}", addr);

            let listener = tokio::net::TcpListener::bind(&addr).await?;
            axum::serve(listener, app).await?;
        }
    }

    Ok(())
}
