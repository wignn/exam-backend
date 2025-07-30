use crate::errors::{AppError, AppResult};
use crate::middleware::auth::AuthUser;
use crate::models::user::{ChangePasswordRequest, UpdateProfileRequest, VerifyEmailRequest};
use crate::services::user::UserService;
use crate::utils::jwt::JwtService;
use crate::AppState;
use axum::{extract::State, response::Json, Extension};
use serde_json::{json, Value};
use validator::Validate;

pub struct UserHandlers;

impl UserHandlers {
    pub async fn get_profile(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
    ) -> AppResult<Json<Value>> {
        let jwt_service = JwtService::new(
            &state.config.jwt_secret,
            state.config.jwt_access_expires_in,
            state.config.jwt_refresh_expires_in,
        );

        let user_service = UserService::new(state.db.clone(), jwt_service);
        let user = user_service.get_user_by_id(auth_user.id).await?;

        Ok(Json(json!({
            "message": "Profile retrieved successfully",
            "data": user
        })))
    }

    pub async fn update_profile(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Json(request): Json<UpdateProfileRequest>,
    ) -> AppResult<Json<Value>> {
        // Validate request
        request.validate().map_err(|e| {
            AppError::Validation(format!("Validation error: {}", e))
        })?;

        let jwt_service = JwtService::new(
            &state.config.jwt_secret,
            state.config.jwt_access_expires_in,
            state.config.jwt_refresh_expires_in,
        );

        let user_service = UserService::new(state.db.clone(), jwt_service);
        let updated_user = user_service.update_profile(auth_user.id, request).await?;

        Ok(Json(json!({
            "message": "Profile updated successfully",
            "data": updated_user
        })))
    }

    pub async fn change_password(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Json(request): Json<ChangePasswordRequest>,
    ) -> AppResult<Json<Value>> {
        // Validate request
        request.validate().map_err(|e| {
            AppError::Validation(format!("Validation error: {}", e))
        })?;

        let jwt_service = JwtService::new(
            &state.config.jwt_secret,
            state.config.jwt_access_expires_in,
            state.config.jwt_refresh_expires_in,
        );

        let user_service = UserService::new(state.db.clone(), jwt_service);
        user_service.change_password(auth_user.id, request).await?;

        Ok(Json(json!({
            "message": "Password changed successfully"
        })))
    }

    pub async fn verify_email(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Json(_request): Json<VerifyEmailRequest>,
    ) -> AppResult<Json<Value>> {
        // In a real implementation, you would:
        // 1. Verify the token from the request
        // 2. Check if it's valid and not expired
        // For now, we'll just mark the user as verified

        let jwt_service = JwtService::new(
            &state.config.jwt_secret,
            state.config.jwt_access_expires_in,
            state.config.jwt_refresh_expires_in,
        );

        let user_service = UserService::new(state.db.clone(), jwt_service);
        let updated_user = user_service.verify_email(auth_user.id).await?;

        Ok(Json(json!({
            "message": "Email verified successfully",
            "data": updated_user
        })))
    }
}