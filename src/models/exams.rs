/*
CREATE TABLE IF NOT EXISTS exams (
    UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title            TEXT      NOT NULL,
    description      TEXT,
    created_by       UUID REFERENCES users (id),
    duration_minutes INTEGER   NOT NULL,
    start_time       TIMESTAMPTZ NOT NULL,
    end_time         TIMESTAMPTZ NOT NULL,
    is_active        BOOLEAN     DEFAULT TRUE
    );
*/
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Exam {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_by: String,
    pub duration_minutes: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub is_active: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExamRequest{
    pub title: String,
    pub description: String,
    pub created_by: String,
    pub duration_minutes: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExamUpdateRequest {
    pub title: String,
    pub description: String,
    pub duration_minutes: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>
}

pub struct ExamResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_by: String,
    pub duration_minutes: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub is_active: bool,
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
            is_active: exam.is_active
        }
    }
}



/**
CREATE TABLE IF NOT EXISTS exam_assignments (
    exam_id  UUID REFERENCES exams (id),
    class_id UUID REFERENCES classes (id),
    PRIMARY KEY (exam_id, class_id)
    );
*/

#[derive(Deserialize, Debug)]
pub struct ExamAssignments {
    pub exam_id: Uuid,
    pub class_id: Uuid
}