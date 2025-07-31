use crate::database::Database;
use crate::errors::{AppError, AppResult};
use crate::models::class::{
    ClassMember, ClassResponse, CreateClassMemberRequest, CreateClassRequest,
    DeleteClassMemberRequest, UpdateClassRequest,
};
use chrono::Utc;
use tracing::log::__private_api::Value;
use uuid::Uuid;

pub struct ClassService {
    db: Database,
}

impl ClassService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
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
        let classes = sqlx::query_as::<_, ClassResponse>(
            "SELECT id, name, created_by, created_at FROM classes",
        )
        .fetch_all(&self.db.pool)
        .await?;

        Ok(classes)
    }
    pub async fn get_class_by_id(&self, class_id: Uuid) -> AppResult<ClassResponse> {
        let result = sqlx::query_as::<_, ClassResponse>(
            "SELECT id, name, created_by, created_at FROM classes WHERE id = $1",
        )
        .bind(class_id)
        .fetch_optional(&self.db.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(result)
    }

    pub async fn updat_class(
        &self,
        class_id: Uuid,
        request: UpdateClassRequest,
    ) -> AppResult<ClassResponse> {
        let class = sqlx::query_as::<_, ClassResponse>(
            "UPDATE classes SET name = $1 WHERE id = $2 RETURNING id, name, created_by, created_at",
        )
        .bind(&request.name)
        .bind(class_id)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(class)
    }

    pub async fn delete_class(&self, class_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM classes WHERE id = $1")
            .bind(class_id)
            .execute(&self.db.pool)
            .await?;
        Ok(())
    }

    pub async fn create_class_member(&self, request: CreateClassMemberRequest) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO class_members (user_id, class_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(&request.user_id)
        .bind(&request.class_id)
        .execute(&self.db.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_class_member(&self, request: DeleteClassMemberRequest) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM class_members WHERE user_id = $1 AND class_id = $2")
            .bind(request.user_id)
            .bind(request.class_id)
            .execute(&self.db.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::errors::AppError::NotFound(
                "Class member not found".to_string(),
            ));
        }

        Ok(())
    }

    pub async fn get_class_members_by_class_id(&self, class_id: Uuid) -> AppResult<Vec<ClassMember>> {
        let result = sqlx::query_as::<_, ClassMember>(
            "SELECT user_id, class_id FROM class_members WHERE class_id = $1",
        )
            .bind(class_id)
            .fetch_all(&self.db.pool)
            .await?;

        Ok(result)
    }
}
