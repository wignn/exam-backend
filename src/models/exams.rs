use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::{Validate};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Exam {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_by: Uuid,
    pub duration_minutes: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub is_active: bool,
    pub category: String,
    pub difficulty: String
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateExamRequest {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub description: String,
    #[validate(range(min = 1))]
    pub duration_minutes: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub category: String,
    pub difficulty: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateExamRequest {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub description: String,
    pub is_active: bool,
    #[validate(range(min = 1))]
    pub duration_minutes: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub category: String,
    pub difficulty: String
}

#[derive(Debug, Serialize, Clone)]
pub struct ExamResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_by: Uuid,
    pub duration_minutes: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub is_active: bool,
    pub category: String,
    pub difficulty: String,
}

impl From<Exam> for ExamResponse {
    fn from(exam: Exam) -> Self {
        Self {
            id: exam.id,
            title: exam.title,
            description: exam.description,
            created_by: exam.created_by,
            duration_minutes: exam.duration_minutes,
            start_time: exam.start_time,
            end_time: exam.end_time,
            is_active: exam.is_active,
            category: exam.category,
            difficulty: exam.difficulty,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExamAssignments {
    pub exam_id: Uuid,
    pub class_id: Uuid,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct CreateExamAssignmentRequest {
    pub exam_id: Uuid,
    pub class_id: Uuid,
}
#[derive(Debug, Deserialize, Clone)]
pub struct DeleteExamAssignmentRequest {
    pub exam_id: Uuid,
    pub class_id: Uuid,
}



#[derive(Debug, Serialize, Clone)]
pub struct ExamAssignmentsResponse {
    pub exam_id: Uuid,
    pub class_id: Uuid,
}

impl From<ExamAssignments> for ExamAssignmentsResponse {
    fn from(e: ExamAssignments) -> Self {
        Self {
            exam_id: e.exam_id,
            class_id: e.class_id,
        }
    }
}
