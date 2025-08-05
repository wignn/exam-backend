use axum::{
    Extension, Json,
    extract::{Path, State},
};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{
    AppState,
    errors::AppResult,
    middleware::auth::AuthUser,
    models::{
        attempt::{
            ExamAttemptResponse, StartExamAttemptRequest,
            SubmitExamAttemptRequest,
        },
        user::UserRole,
    },
    require_role,
    services::exam_attempt::ExamAttemptService,
};

pub struct ExamAttemptHandler;

impl ExamAttemptHandler {
    pub async fn start_exam_attempt(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Json(request): Json<StartExamAttemptRequest>,
    ) -> AppResult<Json<Value>> {
        let exam_attempt_service = ExamAttemptService::new(state.db.clone());
        let exam_attempt = exam_attempt_service
            .start_exam_attempt(request, auth_user.id)
            .await?;
        Ok(Json(json!({
            "message": "Exam attempt started successfully",
            "data": ExamAttemptResponse::from(exam_attempt)
        })))
    }

    pub async fn submit_exam_attempt(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Json(request): Json<SubmitExamAttemptRequest>,
    ) -> AppResult<Json<Value>> {
        let exam_attempt_service = ExamAttemptService::new(state.db.clone());
        let exam_attempt = exam_attempt_service
            .submit_exam_attempt(request, auth_user.id)
            .await?;

        Ok(Json(json!({
              "message": "Exam attempt submitted successfully",
              "data": ExamAttemptResponse::from(exam_attempt)
        })))
    }

    pub async fn get_user_attempts(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
    ) -> AppResult<Json<Value>> {
        let exam_attempt_service = ExamAttemptService::new(state.db.clone());
        let attempts = exam_attempt_service.get_user_attempts(auth_user.id).await?;

        Ok(Json(json!({
            "message": "User exam attempts retrieved successfully",
            "data": attempts
        })))
    }

    pub async fn get_attempt_with_answers(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(attempt_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        let exam_attempt_service = ExamAttemptService::new(state.db.clone());
        let attempt_with_answers = exam_attempt_service
            .get_attempt_with_answers(attempt_id, auth_user.id)
            .await?;

        Ok(Json(json!({
            "message": "Exam attempt with answers retrieved successfully",
            "data": attempt_with_answers
        })))
    }

    //only for teachers
    pub async fn get_exam_attempts(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(exam_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        require_role!(auth_user, UserRole::Teacher)?;
        let exam_attempt_service = ExamAttemptService::new(state.db.clone());
        let attempts = exam_attempt_service.get_exam_attempts(exam_id).await?;

        Ok(Json(json!({
            "message": "Exam attempts retrieved successfully",
            "data": attempts
        })))
    }

    pub async fn get_active_attempt(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(exam_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        let exam_attempt_service = ExamAttemptService::new(state.db.clone());
        let active_attempt = exam_attempt_service.get_active_attempt(auth_user.id, exam_id).await?;

        Ok(Json(json!({
            "message": "Active exam attempt retrieved successfully",
            "data": active_attempt
        })))
    }



}
