use axum::{
    Extension, Json,
    extract::{Path, State},
};
use serde_json::{Value, json};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    errors::{AppError, AppResult},
    middleware::auth::AuthUser,
    models::{
        question::{
            BulkCreateQuestionsRequest, CreateQuestionRequest, UpdateQuestionRequest,
        },
        user::UserRole,
    },
    require_role,
    services::question::QuestionService,
};

pub struct QuestionHandler;

impl QuestionHandler {
    /// Create a single question for an exam (Teacher only)
    pub async fn create_question(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(exam_id): Path<Uuid>,
        Json(request): Json<CreateQuestionRequest>,
    ) -> AppResult<Json<Value>> {
        request
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;
        
        require_role!(auth_user, UserRole::Teacher)?;

        let question_service = QuestionService::new(state.db.clone());
        let question = question_service.create_question(exam_id, request).await?;

        Ok(Json(json!({
            "message": "Question created successfully",
            "data": question
        })))
    }

    /// Get all questions for an exam (Teacher/Admin view with correct answers)
    pub async fn get_questions_by_exam(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(exam_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        require_role!(auth_user, UserRole::Teacher)?;

        let question_service = QuestionService::new(state.db.clone());
        let questions = question_service.get_questions_by_exam(exam_id).await?;

        Ok(Json(json!({
            "message": "Questions retrieved successfully",
            "data": questions
        })))
    }

    /// Get questions for students (without correct answers)
    pub async fn get_questions_for_student(
        State(state): State<AppState>,
        Extension(_auth_user): Extension<AuthUser>,
        Path(exam_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        let question_service = QuestionService::new(state.db.clone());
        let questions = question_service.get_questions_for_student(exam_id).await?;

        Ok(Json(json!({
            "message": "Questions retrieved successfully",
            "data": questions
        })))
    }

    /// Get a single question by ID (Teacher only)
    pub async fn get_question_by_id(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(question_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        require_role!(auth_user, UserRole::Teacher)?;

        let question_service = QuestionService::new(state.db.clone());
        let question = question_service.get_question_by_id(question_id).await?;

        Ok(Json(json!({
            "message": "Question retrieved successfully",
            "data": question
        })))
    }

    /// Update a question (Teacher only)
    pub async fn update_question(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(question_id): Path<Uuid>,
        Json(request): Json<UpdateQuestionRequest>,
    ) -> AppResult<Json<Value>> {
        request
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;
        
        require_role!(auth_user, UserRole::Teacher)?;

        let question_service = QuestionService::new(state.db.clone());
        let question = question_service.update_question(question_id, request).await?;

        Ok(Json(json!({
            "message": "Question updated successfully",
            "data": question
        })))
    }

    /// Delete a question (Teacher only)
    pub async fn delete_question(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(question_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        require_role!(auth_user, UserRole::Teacher)?;

        let question_service = QuestionService::new(state.db.clone());
        question_service.delete_question(question_id).await?;

        Ok(Json(json!({
            "message": "Question deleted successfully"
        })))
    }

    /// Bulk create questions for an exam (Teacher only)
    pub async fn bulk_create_questions(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Json(request): Json<BulkCreateQuestionsRequest>,
    ) -> AppResult<Json<Value>> {
        // Validate all questions
        for question in &request.questions {
            question
                .validate()
                .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;
        }
        
        require_role!(auth_user, UserRole::Teacher)?;

        let question_service = QuestionService::new(state.db.clone());
        let questions = question_service.bulk_create_questions(request).await?;

        Ok(Json(json!({
            "message": "Questions created successfully",
            "data": questions,
            "count": questions.len()
        })))
    }

    /// Get total score for an exam (Teacher only)
    pub async fn get_exam_total_score(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(exam_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        require_role!(auth_user, UserRole::Teacher)?;

        let question_service = QuestionService::new(state.db.clone());
        let total_score = question_service.get_exam_total_score(exam_id).await?;

        Ok(Json(json!({
            "message": "Exam total score retrieved successfully",
            "data": {
                "exam_id": exam_id,
                "total_score": total_score
            }
        })))
    }
}
