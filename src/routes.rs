use crate::handlers::{auth::AuthHandlers, class::ClassHandlers, user::UserHandlers, exam::ExamHandlers};
use crate::middleware::auth::auth_middleware;
use crate::{AppState};
use axum::{
    Router,
    routing::{get, post},
};
use axum::routing::{delete, put};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

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
        .nest("/users", user_routes(state.clone()))
        .nest("/classes", classes_routes(state.clone()))
        .nest("/exams", exams_routes(state.clone()))
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

fn classes_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(ClassHandlers::create_class))
        .route("/", get(ClassHandlers::get_classes))
        .route("/{class_id}", get(ClassHandlers::get_class_by_id))
        .route("/{class_id}", put(ClassHandlers::update_class))
        .route("/{class_id}", delete(ClassHandlers::delete_class))
        .route("/member", post(ClassHandlers::create_class_member))
        .route("/member", delete(ClassHandlers::delete_class_member))
        .route("/member/{class_id}", get(ClassHandlers::get_class_member_by_class_id))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
}

fn exams_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(ExamHandlers::create_exam))
        .route("/", get(ExamHandlers::get_exams))
        .route("/{exam_id}", get(ExamHandlers::get_exam_by_id))
        .route("/{exam_id}", put(ExamHandlers::update_exam))
        .route("/{exam_id}", delete(ExamHandlers::delete_exam))
        .route("/assignments", post(ExamHandlers::assign_exam_to_class))
        .route("/assignments", delete(ExamHandlers::unassign_exam_from_class))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
}

async fn health_check() -> &'static str {
    "OK"
}