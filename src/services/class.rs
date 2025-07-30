use chrono::Utc;
use crate::database::Database;
use crate::errors::AppResult;
use crate::utils::jwt::JwtService;
use sqlx::Row;
use uuid::Uuid;
use crate::models::class::{ClassResponse, CreateClassRequest};

pub struct ClassService {
    db: Database,
    jwt_service: JwtService
}

impl ClassService {
    pub fn new(db: Database, jwt_service: JwtService)-> Self{Self { db, jwt_service }}
    pub async fn create_class(&self, resquest: CreateClassRequest) -> AppResult<()> {
        sqlx::query("INSERT INTO classes (name, created_at, created_by) VALUES ($1, $2, $3)")
            .bind(&resquest.name)
            .bind(Utc::now())
            .bind(&resquest.created_by)
            .execute(&self.db.pool)
            .await?;
        Ok(())
    }

pub async fn get_classes(&self) -> AppResult<Vec<ClassResponse>> {
        let classes = sqlx::query_as::<_, ClassResponse>("SELECT id, name, created_by, created_at FROM classes")
            .fetch_all(&self.db.pool)
            .await?;

        Ok(classes)
    }
    pub async fn get_class_by_id(&self, class_id: Uuid) -> AppResult<ClassResponse> {
        let class = sqlx::query_as::<_, ClassResponse>(
            "SELECT id, name, created_by, created_at FROM classes WHERE id = $1"
        )
            .bind(class_id)
            .fetch_one(&self.db.pool)
            .await?;
        Ok(class)
    }

}


