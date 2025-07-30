use crate::handlers::auth::AuthHandlers;
use crate::handlers::user::UserHandlers;
use crate::middleware::auth::auth_middleware;
use crate::{AppState, health_check};
use axum::{
    Router,
    routing::{get, post},
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{CorsLayer},
    trace::TraceLayer,
};

pub fn create_routes(state: AppState, cors: CorsLayer) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", api_routes(state.clone()))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors),
        )
}

fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/users", user_routes(state))
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(AuthHandlers::register))
        .route("/login", post(AuthHandlers::login))
        .route("/refresh", post(AuthHandlers::refresh_token))
        .route("/logout", post(AuthHandlers::logout))
}

fn user_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/profile", get(UserHandlers::get_profile))
        .route("/profile", post(UserHandlers::update_profile))
        .route("/verify-email", post(UserHandlers::verify_email))
        .route("/change-password", post(UserHandlers::change_password))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
}