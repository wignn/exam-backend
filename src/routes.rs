use crate::handlers::{auth::AuthHandlers, class::ClassHandlers, user::UserHandlers, exam::ExamHandlers, exam_attempt::ExamAttemptHandler, question::QuestionHandler, progress::ProgressHandler};
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
        .nest("/exam-attempts", exam_attempts_routes(state.clone()))
        .nest("/questions", questions_routes(state.clone()))
        .nest("/progress", progress_routes(state.clone()))
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

fn exam_attempts_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/start", post(ExamAttemptHandler::start_exam_attempt))
        .route("/submit", post(ExamAttemptHandler::submit_exam_attempt))
        .route("/my-attempts", get(ExamAttemptHandler::get_user_attempts))
        .route("/details/{attempt_id}", get(ExamAttemptHandler::get_attempt_with_answers))
        .route("/exam/{exam_id}", get(ExamAttemptHandler::get_exam_attempts)) // for teachers
        .route("/active/{exam_id}", get(ExamAttemptHandler::get_active_attempt))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
}

fn questions_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/exam/{exam_id}", post(QuestionHandler::create_question)) 
        .route("/exam/{exam_id}/teacher", get(QuestionHandler::get_questions_by_exam)) 
        .route("/exam/{exam_id}/student", get(QuestionHandler::get_questions_for_student)) 
        .route("/{question_id}", get(QuestionHandler::get_question_by_id)) 
        .route("/{question_id}", put(QuestionHandler::update_question)) 
        .route("/{question_id}", delete(QuestionHandler::delete_question))
        .route("/bulk", post(QuestionHandler::bulk_create_questions)) 
        .route("/exam/{exam_id}/total-score", get(QuestionHandler::get_exam_total_score)) 
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
}

fn progress_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(ProgressHandler::create_progress))
        .route("/{progress_id}", put(ProgressHandler::update_progress))
        .route("/my-progress", get(ProgressHandler::get_user_progress))
        .route("/my-level", get(ProgressHandler::get_user_level))
        .route("/summary", get(ProgressHandler::get_progress_summary))
        .route("/user/{user_id}/progress", get(ProgressHandler::get_user_progress_by_id)) // Teachers only
        .route("/user/{user_id}/level", get(ProgressHandler::get_user_level_by_id)) // Teachers only
        .route("/leaderboard", get(ProgressHandler::get_leaderboard))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
}

async fn health_check() -> &'static str {
    "OK"
}