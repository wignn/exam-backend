use axum::{Extension, Json, extract::{Path, Query, State}};
use serde_json::{Value, json};
use sqlx::Row;
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    errors::{AppError, AppResult},
    middleware::auth::AuthUser,
    models::progress::{CreateProgressRequest, UpdateProgressRequest},
    services::progress::ProgressService,
};

pub struct ProgressHandler;

impl ProgressHandler {
    /// Create a new progress entry
    pub async fn create_progress(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Json(request): Json<CreateProgressRequest>,
    ) -> AppResult<Json<Value>> {
        request
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

        let progress_service = ProgressService::new(state.db.clone());
        let progress = progress_service
            .create_progress(auth_user.id, request)
            .await?;

        Ok(Json(json!({
            "message": "Progress entry created successfully",
            "data": progress
        })))
    }

    /// Update existing progress
    pub async fn update_progress(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(progress_id): Path<Uuid>,
        Json(request): Json<UpdateProgressRequest>,
    ) -> AppResult<Json<Value>> {
        request
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

        let progress_service = ProgressService::new(state.db.clone());
        let progress = progress_service
            .update_progress(progress_id, auth_user.id, request)
            .await?;

        Ok(Json(json!({
            "message": "Progress updated successfully",
            "data": progress
        })))
    }

    /// Get user's progress history
    pub async fn get_user_progress(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Query(params): Query<ProgressQueryParams>,
    ) -> AppResult<Json<Value>> {
        let progress_service = ProgressService::new(state.db.clone());
        let progress_list = progress_service
            .get_user_progress(auth_user.id, params.limit)
            .await?;

        Ok(Json(json!({
            "message": "User progress retrieved successfully",
            "data": progress_list
        })))
    }

    /// Get user level and experience info
    pub async fn get_user_level(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
    ) -> AppResult<Json<Value>> {
        let progress_service = ProgressService::new(state.db.clone());
        let user_level = progress_service.get_user_level(auth_user.id).await?;

        Ok(Json(json!({
            "message": "User level retrieved successfully",
            "data": user_level
        })))
    }

    /// Get progress summary for dashboard
    pub async fn get_progress_summary(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
    ) -> AppResult<Json<Value>> {
        let progress_service = ProgressService::new(state.db.clone());
        let summary = progress_service.get_progress_summary(auth_user.id).await?;

        Ok(Json(json!({
            "message": "Progress summary retrieved successfully",
            "data": summary
        })))
    }

    /// Get specific user's progress (for teachers/admins)
    pub async fn get_user_progress_by_id(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(user_id): Path<Uuid>,
        Query(params): Query<ProgressQueryParams>,
    ) -> AppResult<Json<Value>> {
        // Only teachers and admins can view other users' progress
        use crate::models::user::UserRole;
        use crate::require_role;
        require_role!(auth_user, UserRole::Teacher)?;

        let progress_service = ProgressService::new(state.db.clone());
        let progress_list = progress_service
            .get_user_progress(user_id, params.limit)
            .await?;

        Ok(Json(json!({
            "message": "User progress retrieved successfully",
            "data": progress_list
        })))
    }

    /// Get user level by ID (for teachers/admins)
    pub async fn get_user_level_by_id(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Path(user_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        // Only teachers and admins can view other users' levels
        use crate::models::user::UserRole;
        use crate::require_role;
        require_role!(auth_user, UserRole::Teacher)?;

        let progress_service = ProgressService::new(state.db.clone());
        let user_level = progress_service.get_user_level(user_id).await?;

        Ok(Json(json!({
            "message": "User level retrieved successfully",
            "data": user_level
        })))
    }

    /// Get leaderboard (top users by level/experience)
    pub async fn get_leaderboard(
        State(state): State<AppState>,
        Query(params): Query<LeaderboardQueryParams>,
    ) -> AppResult<Json<Value>> {
        let limit = params.limit.unwrap_or(10).min(50); // Max 50 users
        
        let rows = sqlx::query(
            r#"
            SELECT ul.user_id, ul.current_level, ul.total_experience, ul.level_title, u.name
            FROM user_levels ul
            JOIN users u ON ul.user_id = u.id
            ORDER BY ul.current_level DESC, ul.total_experience DESC
            LIMIT $1
            "#
        )
        .bind(limit)
        .fetch_all(&state.db.pool)
        .await?;

        let leaderboard: Vec<Value> = rows.into_iter().map(|row| {
            json!({
                "user_id": row.get::<Uuid, _>("user_id"),
                "name": row.get::<String, _>("name"),
                "level": row.get::<i32, _>("current_level"),
                "total_experience": row.get::<i32, _>("total_experience"),
                "level_title": row.get::<String, _>("level_title")
            })
        }).collect();

        Ok(Json(json!({
            "message": "Leaderboard retrieved successfully",
            "data": leaderboard
        })))
    }
}

#[derive(serde::Deserialize)]
pub struct ProgressQueryParams {
    pub limit: Option<i32>,
}

#[derive(serde::Deserialize)]
pub struct LeaderboardQueryParams {
    pub limit: Option<i32>,
}
