use crate::database::Database;
use crate::errors::AppResult;
use crate::models::exams::{Exam, ExamRequest, ExamResponse};
use crate::services::user::UserService;
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

    pub async fn create_exam(&self, request: ExamRequest) -> AppResult<ExamResponse> {
        let row = sqlx::query(
            r#"
            Insert Into exams (title, description, created_by, duration_minutes, start_time, end_time, isActive)
        VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, title, description, created_by, duration_minutes, start_time, end_time, isActive
            "#)
            .bind(&request.title)
            .bind(&request.description)
            .bind(&request.created_by)
            .bind(&request.duration_minutes)
            .bind(&request.start_time)
            .bind(&request.end_time)
            .bind(true)
            .fetch_one(&self.db.pool)
            .await?;

        let exam = self.row_to_exam(row)?;
        Ok(exam.into())
    }

    pub async fn get_exams(&self) -> AppResult<Vec<ExamResponse>> {
        let rows = sqlx::query(
            r#"
        SELECT id, title, description, created_by, duration_minutes, start_time, end_time, is_active
        FROM exams
        "#)
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


    fn row_to_exam(&self, row: PgRow) -> Result<Exam, sqlx::Error> {
        Ok(Exam {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            created_by: row.get("created_by"),
            duration_minutes: row.get("duration_minutes"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            is_active: row.get("isActive"),
        })
    }
}
