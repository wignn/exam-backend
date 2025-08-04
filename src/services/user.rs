use crate::database::Database;
use crate::errors::{AppError, AppResult};
use crate::models::user::{
    AuthResponse, ChangePasswordRequest, LoginRequest, RegisterRequest,
    UpdateProfileRequest, User, UserResponse
};
use crate::utils::jwt::JwtService;
use crate::utils::password::PasswordService;
use chrono::{Utc};
use sqlx::{Row, postgres::PgRow};
use uuid::Uuid;

pub struct UserService {
    db: Database,
    jwt_service: JwtService,
}

impl UserService {
    pub fn new(db: Database, jwt_service: JwtService) -> Self {
        Self { db, jwt_service }
    }

    pub async fn register(&self, request: RegisterRequest) -> AppResult<AuthResponse> {
        // Check if user already exists
        let existing_user = sqlx::query("SELECT id FROM users WHERE email = $1")
            .bind(&request.email)
            .fetch_optional(&self.db.pool)
            .await?;

        if existing_user.is_some() {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        // Hash password
        let hashed_password = PasswordService::hash_password(&request.password)?;

        // Insert new user
        let row = sqlx::query(
            r#"
            INSERT INTO users (name, email, password, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, email, password, is_active, is_verified, role, created_at, updated_at
            "#
        )
            .bind(&request.name)
            .bind(&request.email)
            .bind(&hashed_password)
            .bind(&request.role)
            .bind(Utc::now())
            .bind(Utc::now())
            .fetch_one(&self.db.pool)
            .await?;

        let user = self.row_to_user(row)?;

        // Generate tokens
        let access_token = self.jwt_service.generate_access_token(
            user.id,
            &user.email,
            user.role.clone(),
        )?;

        let refresh_token = self.jwt_service.generate_refresh_token(
            user.id,
            &user.email,
            user.role.clone(),
        )?;

        Ok(AuthResponse {
            user: user.into(),
            access_token,
            refresh_token,
        })
    }

    pub async fn login(&self, request: LoginRequest) -> AppResult<AuthResponse> {
        // Find user by email
        let row = sqlx::query(
            r#"
            SELECT id, name, email, password, is_active, is_verified, role, created_at, updated_at
            FROM users
            WHERE email = $1
            "#
        )
            .bind(&request.email)
            .fetch_optional(&self.db.pool)
            .await?
            .ok_or_else(|| AppError::Unauthorized)?;

        let user = self.row_to_user(row)?;

        // Check if user is active
        if !user.is_active {
            return Err(AppError::Forbidden);
        }

        // Verify password
        if !PasswordService::verify_password(&request.password, &user.password)? {
            return Err(AppError::Unauthorized);
        }

        // Generate tokens
        let access_token = self.jwt_service.generate_access_token(
            user.id,
            &user.email,
            user.role.clone(),
        )?;

        let refresh_token = self.jwt_service.generate_refresh_token(
            user.id,
            &user.email,
            user.role.clone(),
        )?;

        Ok(AuthResponse {
            user: user.into(),
            access_token,
            refresh_token,
        })
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> AppResult<AuthResponse> {
        // Verify refresh token
        let claims = self.jwt_service.verify_refresh_token(refresh_token)?;
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized)?;

        // Get user from database
        let row = sqlx::query(
            r#"
            SELECT id, name, email, password, is_active, is_verified, role, created_at, updated_at
            FROM users
            WHERE id = $1 AND is_active = true
            "#
        )
            .bind(user_id)
            .fetch_optional(&self.db.pool)
            .await?
            .ok_or_else(|| AppError::Unauthorized)?;

        let user = self.row_to_user(row)?;

        // Generate new tokens
        let access_token = self.jwt_service.generate_access_token(
            user.id,
            &user.email,
            user.role.clone(),
        )?;

        let new_refresh_token = self.jwt_service.generate_refresh_token(
            user.id,
            &user.email,
            user.role.clone(),
        )?;

        Ok(AuthResponse {
            user: user.into(),
            access_token,
            refresh_token: new_refresh_token,
        })
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> AppResult<UserResponse> {
        let row = sqlx::query(
            r#"
            SELECT id, name, email, password, is_active, is_verified, role, created_at, updated_at
            FROM users
            WHERE id = $1
            "#)
            .bind(user_id)
            .fetch_optional(&self.db.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let user = self.row_to_user(row)?;
        Ok(user.into())
    }

    pub async fn update_profile(
        &self,
        user_id: Uuid,
        request: UpdateProfileRequest,
    ) -> AppResult<UserResponse> {
        if request.name.is_none() {
            return self.get_user_by_id(user_id).await;
        }

        let row = sqlx::query(
            r#"
            UPDATE users
            SET name = COALESCE($1, name), updated_at = $2
            WHERE id = $3
            RETURNING id, name, email, password, is_active, is_verified, role, created_at, updated_at
            "#
        )
            .bind(&request.name)
            .bind(Utc::now())
            .bind(user_id)
            .fetch_one(&self.db.pool)
            .await?;

        let user = self.row_to_user(row)?;
        Ok(user.into())
    }

    pub async fn change_password(
        &self,
        user_id: Uuid,
        request: ChangePasswordRequest,
    ) -> AppResult<()> {
        // Get current user
        let row = sqlx::query("SELECT password FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.db.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let current_password: String = row.get("password");

        // Verify current password
        if !PasswordService::verify_password(&request.current_password, &current_password)? {
            return Err(AppError::Unauthorized);
        }

        // Hash new password
        let new_hashed_password = PasswordService::hash_password(&request.new_password)?;

        // Update password in database
        sqlx::query("UPDATE users SET password = $1, updated_at = $2 WHERE id = $3")
            .bind(&new_hashed_password)
            .bind(Utc::now())
            .bind(user_id)
            .execute(&self.db.pool)
            .await?;

        Ok(())
    }

    pub async fn verify_email(&self, user_id: Uuid) -> AppResult<UserResponse> {
        let row = sqlx::query(
            r#"
            UPDATE users
            SET is_verified = true, updated_at = $1
            WHERE id = $2
            RETURNING id, name, email, password, is_active, is_verified, role, created_at, updated_at
            "#
        )
            .bind(Utc::now())
            .bind(user_id)
            .fetch_optional(&self.db.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let user = self.row_to_user(row)?;
        Ok(user.into())
    }

    // Helper function to convert database row to User struct
    fn row_to_user(&self, row: PgRow) -> Result<User, sqlx::Error> {
        Ok(User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            password: row.get("password"),
            is_active: row.get("is_active"),
            is_verified: row.get("is_verified"),
            role: row.get("role"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}