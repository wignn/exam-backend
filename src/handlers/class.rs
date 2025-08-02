use crate::AppState;
use crate::errors::{AppError, AppResult};
use crate::middleware::auth::AuthUser;
use crate::models::user::UserRole;
use crate::services::class::ClassService;
use axum::{Extension, extract::Path, extract::State, response::Json};
use serde_json::{Value, json};
use uuid::Uuid;
use validator::Validate;

pub struct ClassHandlers;
use crate::models::class::{
    CreateClassMemberRequest, CreateClassRequest, DeleteClassMemberRequest, UpdateClassRequest,
};
use crate::require_role;
use crate::services::user::UserService;
use crate::utils::jwt::JwtService;

impl ClassHandlers {
    pub async fn get_classes(State(state): State<AppState>) -> AppResult<Json<Value>> {
        let class_service = ClassService::new(state.db.clone());
        let class = class_service.get_classes().await?;

        Ok(Json(json!({
            "message": "Classes retrieved successfully",
            "class": class
        })))
    }

    pub async fn create_class(
        State(state): State<AppState>,
        Extension(auth_user): Extension<AuthUser>,
        Json(request): Json<CreateClassRequest>,
    ) -> AppResult<Json<Value>> {
        request
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;
        require_role!(auth_user, UserRole::Teacher)?;
        let class_service = ClassService::new(state.db.clone());
        let class = class_service.create_class(request, auth_user.id).await?;

        Ok(Json(json!({

            "message": "Class created successfully" ,
            "class":  class
        })))
    }

    pub async fn get_class_by_id(
        State(state): State<AppState>,
        Path(class_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        let class_service = ClassService::new(state.db.clone());
        let class = class_service.get_class_by_id(class_id).await?;
        Ok(Json(json!({
            "message": "Class retrieved successfully",
            "class": class}
        )))
    }

    pub async fn update_class(
        State(state): State<AppState>,
        Path(class_id): Path<Uuid>,
        Json(request): Json<UpdateClassRequest>,
    ) -> AppResult<Json<Value>> {
        request
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;
        let class_service = ClassService::new(state.db.clone());

        let _exist = class_service
            .get_class_by_id(class_id)
            .await
            .map_err(|_| AppError::NotFound("Class not found".to_string()))?;


        let class = class_service.update_class(class_id, request).await?;

        Ok(Json(json!({
            "message":"Class update successfully",
            "class": class
        })))
    }

    pub async fn delete_class(
        State(state): State<AppState>,
        Path(class_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        let class_service = ClassService::new(state.db.clone());
        let _exist = class_service
            .get_class_by_id(class_id)
            .await
            .map_err(|_| AppError::NotFound("Class not found".to_string()))?;

        class_service.delete_class(class_id).await?;

        Ok(Json(json!({
            "message":"class delete successfully"
        })))
    }

    pub async fn create_class_member(
        State(state): State<AppState>,
        Json(request): Json<CreateClassMemberRequest>,
    ) -> AppResult<Json<Value>> {
        request
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;
        let class_service = ClassService::new(state.db.clone());
        class_service.create_class_member(request).await?;
        Ok(Json(
            json!({ "message": "Class member created successfully" }),
        ))
    }

    pub async fn delete_class_member(
        State(state): State<AppState>,
        Json(request): Json<DeleteClassMemberRequest>,
    ) -> AppResult<Json<Value>> {
        let class_service = ClassService::new(state.db.clone());
        class_service.delete_class_member(request).await?;

        Ok(Json(
            json!({ "message": "Class member deleted successfully" }),
        ))
    }

    pub async fn get_class_member_by_class_id(
        State(state): State<AppState>,
        Path(class_id): Path<Uuid>,
    ) -> AppResult<Json<Value>> {
        let jwt_service = JwtService::new(
            &state.config.jwt_secret,
            state.config.jwt_access_expires_in,
            state.config.jwt_refresh_expires_in,
        );
        let class_service = ClassService::new(state.db.clone());

        let class_member = class_service
            .get_class_members_by_class_id(class_id)
            .await?;

        let user_service = UserService::new(state.db.clone(), jwt_service);
        let mut users = Vec::new();

        for member in class_member {
            if let Ok(user) = user_service.get_user_by_id(member.user_id).await {
                users.push(user);
            }
        }

        Ok(Json(json!(
            {
            "users": users})))
    }
}
