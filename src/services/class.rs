use crate::database::Database;
use crate::errors::{AppError, AppResult};
use crate::models::class::{
    Class, ClassMember, ClassMemberResponse, ClassResponse, CreateClassMemberRequest,
    CreateClassRequest, DeleteClassMemberRequest, UpdateClassRequest,
};
use chrono::Utc;
use sqlx::Row;
use sqlx::postgres::PgRow;
use uuid::Uuid;
use crate::utils::pagination;

pub struct ClassService {
    db: Database,
}

impl ClassService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_class(&self, request: CreateClassRequest, user_id: Uuid) -> AppResult<ClassResponse> {
        let row = sqlx::query(
            r#"
            INSERT INTO classes (name, created_at, created_by)
            VALUES ($1, $2, $3)
            RETURNING id, name, created_by, created_at
        "#,
        )
        .bind(&request.name)
        .bind(Utc::now())
        .bind(user_id)
        .fetch_one(&self.db.pool)
        .await?;

        let class = self.row_to_class(row)?;
        Ok(class.into())
    }

    pub async fn get_classes(&self) -> AppResult<Vec<ClassResponse>> {
        let rows = sqlx::query(r#"SELECT id, name, created_by, created_at FROM classes"#)
            .fetch_all(&self.db.pool)
            .await?;

        let classes = rows
            .into_iter()
            .map(|row| self.row_to_class(row).unwrap().into())
            .collect();

        Ok(classes)
    }

    pub async fn get_classes_paged(&self, pagination: &pagination::Pagination) -> AppResult<Vec<ClassResponse>> {
        let rows = sqlx::query(
            r#"SELECT id, name, created_by, created_at FROM classes ORDER BY created_at DESC LIMIT $1 OFFSET $2"#
        )
        .bind(pagination.limit_or_default(20))
        .bind(pagination.skip_or_default())
        .fetch_all(&self.db.pool)
        .await?;

        let classes = rows
            .into_iter()
            .map(|row| self.row_to_class(row).unwrap().into())
            .collect();

        Ok(classes)
    }

    pub async fn get_class_by_id(&self, class_id: Uuid) -> AppResult<ClassResponse> {
        let row = sqlx::query(r#"SELECT id, name, created_by, created_at FROM classes WHERE id = $1"#)
            .bind(class_id)
            .fetch_optional(&self.db.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Class not found".to_string()))?;

        let class = self.row_to_class(row)?;
        Ok(class.into())
    }

    pub async fn update_class(
        &self,
        class_id: Uuid,
        request: UpdateClassRequest,
    ) -> AppResult<ClassResponse> {
        let row = sqlx::query(
            r#"UPDATE classes SET name = $1 WHERE id = $2 RETURNING id, name, created_by, created_at"#,
        )
        .bind(&request.name)
        .bind(class_id)
        .fetch_one(&self.db.pool)
        .await?;

        let class = self.row_to_class(row)?;
        Ok(class.into())
    }

    pub async fn delete_class(&self, class_id: Uuid) -> AppResult<()> {
        sqlx::query(r#"DELETE FROM classes WHERE id = $1"#)
            .bind(class_id)
            .execute(&self.db.pool)
            .await?;
        Ok(())
    }

    pub async fn create_class_member(&self, request: CreateClassMemberRequest) -> AppResult<()> {
        sqlx::query(
            r#"INSERT INTO class_members (user_id, class_id, created_at) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"#,
        )
        .bind(&request.user_id)
        .bind(&request.class_id)
        .bind(Utc::now())
        .execute(&self.db.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_class_member(&self, request: DeleteClassMemberRequest) -> AppResult<()> {
        let result = sqlx::query(r#"DELETE FROM class_members WHERE user_id = $1 AND class_id = $2"#)
            .bind(request.user_id)
            .bind(request.class_id)
            .execute(&self.db.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Class member not found".to_string()));
        }

        Ok(())
    }

    pub async fn get_class_members_by_class_id(
        &self,
        class_id: Uuid,
    ) -> AppResult<Vec<ClassMemberResponse>> {
        let row = sqlx::query(r#"SELECT user_id, class_id FROM class_members WHERE class_id = $1"#)
            .bind(class_id)
            .fetch_all(&self.db.pool)
            .await?;

        let class_members = row
            .into_iter()
            .map(|row| self.row_to_class_member(row).map(Into::into))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(class_members)
    }

    fn row_to_class(&self, row: PgRow) -> Result<Class, sqlx::Error> {
        Ok(Class {
            id: row.get("id"),
            name: row.get("name"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
        })
    }

    fn row_to_class_member(&self, row: PgRow) -> Result<ClassMember, sqlx::Error> {
        Ok(ClassMember {
            class_id: row.get("class_id"),
            user_id: row.get("user_id"),
        })
    }
}
