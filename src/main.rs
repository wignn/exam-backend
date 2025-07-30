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
    routing::{get, post},
    Router,
};
use config::Config;
use database::Database;
use handlers::auth::AuthHandlers;
use handlers::user::UserHandlers;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub db: Database,
    pub config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "exam_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;

    // Initialize database
    let db = Database::new(&config.database_url).await?;

    //cors
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);



    // Create app state
    let state = Arc::new(AppStateInner { db, config });

    // Create router
    let app = routes::create_routes(state.clone(), cors);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}



async fn health_check() -> &'static str {
    "OK"
}