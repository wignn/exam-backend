use crate::AppState;
use crate::errors::{AppError, AppResult};
use crate::middleware::auth::AuthUser;
use crate::models::user::UserRole;
use crate::services::exam::ExamService;
use axum::{Extension, extract::Path, extract::Query, extract::State, response::Json};
use serde_json::{Value, json};
use uuid::Uuid;
use validator::Validate;

pub struct ExamHandlers;
use crate::models::exams::{
    CreateExamRequest, UpdateExamRequest, CreateExamAssignmentRequest, DeleteExamAssignmentRequest,
};
use crate::require_role;
use crate::utils::pagination::Pagination;

impl ExamHandlers {
    pub async fn get_exams(
        State(state): State<AppState>,
        Query(pagination): Query<Pagination>,
    ) -> AppResult<Json<Value>> {
        let exam_service = ExamService::new(state.db.clone());
        let exams = exam_service.get_exams(&pagination).await?;

        Ok(Json(json!({
            "message": "Exams retrieved successfully",
            "exams": exams
        })))
    }

    pub async fn create_exam(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Json(request): Json<CreateExamRequest>,
    ) -> AppResult<Json<Value>> {
        request
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;
        require_role!(auth_user, UserRole::Teacher)?;


        let exam_service = ExamService::new(state.db.clone());
        let exam = exam_service.create_exam(request, auth_user.id).await?;

        Ok(Json(json!({
            "message": "Exam created successfully",
            "exam": exam
        })))
    }

    pub async fn get_exam_by_id(
        State(state): State<AppState>,
        Path(exam_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        let exam_service = ExamService::new(state.db.clone());
        let exam = exam_service.get_exam_by_id(exam_id).await?;

        Ok(Json(json!({
            "message": "Exam retrieved successfully",
            "exam": exam
        })))
    }

    pub async fn update_exam(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(exam_id): Path<Uuid>,
        Json(request): Json<UpdateExamRequest>,
    ) -> AppResult<Json<Value>> {
        request
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;
        require_role!(auth_user, UserRole::Teacher)?;

        let exam_service = ExamService::new(state.db.clone());

        let _exist = exam_service
            .get_exam_by_id(exam_id)
            .await
            .map_err(|_| AppError::NotFound("Exam not found".to_string()))?;

        let exam = exam_service.update_exam(exam_id, request).await?;

        Ok(Json(json!({
            "message": "Exam updated successfully",
            "exam": exam
        })))
    }

    pub async fn delete_exam(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(exam_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        require_role!(auth_user, UserRole::Teacher)?;

        let exam_service = ExamService::new(state.db.clone());
        let _exist = exam_service
            .get_exam_by_id(exam_id)
            .await
            .map_err(|_| AppError::NotFound("Exam not found".to_string()))?;

        exam_service.delete_exam(exam_id).await?;

        Ok(Json(json!({
            "message": "Exam deleted successfully"
        })))
    }

    pub async fn assign_exam_to_class(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Json(request): Json<CreateExamAssignmentRequest>,
    ) -> AppResult<Json<Value>> {
        request
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;
        require_role!(auth_user, UserRole::Teacher)?;

        let exam_service = ExamService::new(state.db.clone());
        let assignment = exam_service.create_exam_assign(request).await?;

        Ok(Json(json!({
            "message": "Exam assigned to class successfully",
            "assignment": assignment
        })))
    }

    pub async fn unassign_exam_from_class(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Json(request): Json<DeleteExamAssignmentRequest>,
    ) -> AppResult<Json<Value>> {
        require_role!(auth_user, UserRole::Teacher)?;

        let exam_service = ExamService::new(state.db.clone());
        exam_service.delete_exam_assign(request).await?;

        Ok(Json(json!({
            "message": "Exam unassigned from class successfully"
        })))
    }
}
