use crate::errors::{AppError, AppResult};
use crate::models::user::{LoginRequest, RefreshTokenRequest, RegisterRequest};
use crate::services::user::UserService;
use crate::utils::jwt::JwtService;
use crate::AppState;
use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::{json, Value};
use validator::Validate;

pub struct AuthHandlers;

impl AuthHandlers {
    pub async fn register(
        State(state): State<AppState>,
        Json(request): Json<RegisterRequest>,
    ) -> AppResult<(StatusCode, Json<Value>)> {
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
        let auth_response = user_service.register(request).await?;

        Ok((
            StatusCode::CREATED,
            Json(json!({
                "message": "User registered successfully",
                "data": auth_response
            })),
        ))
    }

    pub async fn login(
        State(state): State<AppState>,
        Json(request): Json<LoginRequest>,
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
        let auth_response = user_service.login(request).await?;

        Ok(Json(json!({
            "message": "Login successful",
            "data": auth_response
        })))
    }

    pub async fn refresh_token(
        State(state): State<AppState>,
        Json(request): Json<RefreshTokenRequest>,
    ) -> AppResult<Json<Value>> {
        let jwt_service = JwtService::new(
            &state.config.jwt_secret,
            state.config.jwt_access_expires_in,
            state.config.jwt_refresh_expires_in,
        );

        let user_service = UserService::new(state.db.clone(), jwt_service);
        let auth_response = user_service.refresh_token(&request.refresh_token).await?;

        Ok(Json(json!({
            "message": "Token refreshed successfully",
            "data": auth_response
        })))
    }

    pub async fn logout() -> AppResult<Json<Value>> {
        // In a real implementation, you might want to:
        // 1. Add the token to a blacklist
        // 2. Remove refresh token from database
        // For now, we'll just return a success message
        Ok(Json(json!({
            "message": "Logout successful"
        })))
    }
}