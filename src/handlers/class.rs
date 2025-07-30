use crate::AppState;
use crate::errors::AppResult;
use crate::services::class::ClassService;
use crate::utils::jwt::JwtService;
use axum::{Extension, extract::Path, extract::State, response::Json};
use serde_json::{Value, json};
use uuid::Uuid;
use crate::middleware::auth::AuthUser;
use crate::models::user::UserRole;
pub struct ClassHandlers;
use crate::models::class::{CreateClassRequest, ClassResponse};
use crate::require_role;

impl ClassHandlers {
    pub async fn get_classes(State(state): State<AppState>) -> AppResult<Json<Value>> {
        let jwt_service = JwtService::new(
            &state.config.jwt_secret,
            state.config.jwt_access_expires_in,
            state.config.jwt_refresh_expires_in,
        );

        let class_service = ClassService::new(state.db.clone(), jwt_service);
        let class = class_service.get_classes().await?;

        Ok(Json(json!({
            "message": "Classes retrieved successfully",
            "data": class
        })))
    }

    pub async fn create_class(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Json(mut request): Json<CreateClassRequest>,
    ) -> AppResult<Json<Value>> {
        require_role!(auth_user, UserRole::Teacher)?;
        request.created_by = auth_user.id;

        let jwt_service = JwtService::new(
            &state.config.jwt_secret,
            state.config.jwt_access_expires_in,
            state.config.jwt_refresh_expires_in,
        );

        let class_service = ClassService::new(state.db.clone(), jwt_service);
        class_service.create_class(request).await?;

        Ok(Json(json!({ "message": "Class created successfully" })))
    }

    pub async fn get_class_by_id(
        State(state): State<AppState>,
        Path(class_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        let jwt_service = JwtService::new(
            &state.config.jwt_secret,
            state.config.jwt_access_expires_in,
            state.config.jwt_refresh_expires_in,
        );
        let class_service = ClassService::new(state.db.clone(), jwt_service);
        let class = class_service.get_class_by_id(class_id).await?;

        Ok(Json(json!({
            "message": "Class retrieved successfully",
            "data": class}
        )))
    }
}
