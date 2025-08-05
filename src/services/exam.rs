use crate::database::Database;
use crate::errors::{AppError, AppResult};
use crate::models::exams::{
    CreateExamAssignmentRequest, CreateExamRequest, DeleteExamAssignmentRequest, Exam,
    ExamAssignments, ExamAssignmentsResponse, ExamResponse, UpdateExamRequest,
};
use sqlx::Row;
use sqlx::postgres::PgRow;
use uuid::Uuid;

pub struct ExamService {
    db: Database,
}

impl ExamService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_exam(&self, request: CreateExamRequest, created_by: Uuid) -> AppResult<ExamResponse> {
        let row = sqlx::query(
            r#"
Insert Into exams (title, description, created_by, duration_minutes, start_time, end_time, is_active)
VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, title, description, created_by, duration_minutes, start_time, end_time, is_active
            "#)
            .bind(&request.title)
            .bind(&request.description)
            .bind(&created_by)
            .bind(&request.duration_minutes)
            .bind(&request.start_time)
            .bind(&request.end_time)
            .bind(true)
            .fetch_one(&self.db.pool)
            .await?;

        let exam = self.row_to_exam(row)?;
        Ok(exam.into())
    }

    // pub async fn get_exams(&self) -> AppResult<Vec<ExamResponse>> {
    //     let rows = sqlx::query(
    //         r#"
    //     SELECT id, title, description, created_by, duration_minutes, start_time, end_time, is_active
    //     FROM exams
    //     "#,
    //     )
    //     .fetch_all(&self.db.pool)
    //     .await?;
    //
    //     let exams: Vec<ExamResponse> = rows
    //         .into_iter()
    //         .map(|row| self.row_to_exam(row).unwrap().into())
    //         .collect();
    //     Ok(exams)
    // }

    pub async fn get_exams(&self, pagination: &crate::utils::pagination::Pagination) -> AppResult<Vec<ExamResponse>> {
        let rows = sqlx::query(
            r#"

    SELECT id, title, description, created_by, duration_minutes, start_time, end_time, is_active
    FROM exams
    ORDER BY start_time DESC
    LIMIT $1 OFFSET $2
    "#,
        )
        .bind(pagination.limit_or_default(20))
        .bind(pagination.skip_or_default())
        .fetch_all(&self.db.pool)
        .await?;

        let exams: Vec<ExamResponse> = rows
            .into_iter()
            .map(|row| self.row_to_exam(row).unwrap().into())
            .collect();
        Ok(exams)
    }

    pub async fn get_exam_by_id(&self, id: Uuid) -> AppResult<ExamResponse> {
        let row = sqlx::query(
            r#"
            SELECT id, title, description, created_by, duration_minutes, start_time, end_time, is_active
            FROM exams
            WHERE id = $1
            "#)
            .bind(id)
            .fetch_one(&self.db.pool)
            .await?;

        let exam = self.row_to_exam(row)?;

        Ok(exam.into())
    }

    pub async fn update_exam(
        &self,
        id: Uuid,
        request: UpdateExamRequest,
    ) -> AppResult<ExamResponse> {
        let row = sqlx::query(
        r#"
        UPDATE exams
        SET title = $1, description = $2, duration_minutes = $3, start_time = $4, end_time = $5, isActive = $6
        WHERE id = $7
        RETURNING id, title, description, created_by, duration_minutes, start_time, end_time, isActive
        "#
    )
    .bind(&request.title)
    .bind(&request.description)
    .bind(&request.duration_minutes)
    .bind(&request.start_time)
    .bind(&request.end_time)
    .bind(&request.is_active)
    .bind(id)
    .fetch_one(&self.db.pool)
    .await?;

        let exam = self.row_to_exam(row)?;
        Ok(exam.into())
    }

    pub async fn delete_exam(&self, id: Uuid) -> AppResult<()> {
        let result = sqlx::query(r#"DELETE FROM exams WHERE id = $1"#)
            .bind(id)
            .execute(&self.db.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Exam not found".to_string()));
        }

        Ok(())
    }

    pub async fn create_exam_assign(
        &self,
        request: CreateExamAssignmentRequest,
    ) -> AppResult<ExamAssignmentsResponse> {
        let row = sqlx::query(
            r#"
        INSERT INTO exam_assignments (class_id, exam_id, created_at)
        VALUES ($1, $2, $3)
        RETURNING exam_id, class_id, created_at
        "#,
        )
        .bind(&request.class_id)
        .bind(&request.exam_id)
        .bind(chrono::Utc::now())
        .fetch_one(&self.db.pool)
        .await?;
        let exam_assign = self.row_to_exam_assign(row)?;

        Ok(exam_assign.into())
    }
    pub async fn delete_exam_assign(&self, request: DeleteExamAssignmentRequest) -> AppResult<()> {
        let result = sqlx::query(
            r#"
                DELETE FROM exam_assignments
                WHERE exam_id = $1 AND class_id = $2
            "#,
        )
        .bind(&request.exam_id)
        .bind(&request.class_id)
        .execute(&self.db.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Exam assignment not found".to_string()));
        }

        Ok(())
    }

    fn row_to_exam(&self, row: PgRow) -> Result<Exam, sqlx::Error> {
        Ok(Exam {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            created_by: row.get("created_by"),
            duration_minutes: row.get("duration_minutes"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            is_active: row.get("is_active"),
        })
    }

    fn row_to_exam_assign(&self, row: PgRow) -> Result<ExamAssignments, sqlx::Error> {
        Ok(ExamAssignments {
            exam_id: row.get("exam_id"),
            class_id: row.get("class_id"),
        })
    }
}
