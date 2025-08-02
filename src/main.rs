mod config;
mod database;
mod errors;
mod handlers;
mod middleware;
mod models;
mod services;
mod utils;
mod routes;

use axum::{
    http::{header, Method},
};
use config::Config;
use database::Database;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub db: Database,
    pub config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing (logger)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,exam_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;

    // Initialize database
    let db = Database::new(&config.database_url).await?;

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    // Create shared application state
    let state = Arc::new(AppStateInner { db, config });

    // Build the router
    let app = routes::create_routes(state.clone(), cors);

    // Start server
    let addr = "0.0.0.0:3000";
    tracing::info!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

