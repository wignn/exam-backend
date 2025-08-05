use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExamAttempt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub exam_id: Uuid,
    pub started_at: Option<DateTime<Utc>>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub score_total: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct StartExamAttemptRequest {
    pub exam_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SubmitExamAttemptRequest {
    pub attempt_id: Uuid,
    pub answers: Vec<AnswerSubmission>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AnswerSubmission {
    pub question_id: Uuid,
    pub answer_text: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ExamAttemptResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub exam_id: Uuid,
    pub started_at: Option<DateTime<Utc>>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub score_total: Option<i32>,
    pub status: String,
}

impl From<ExamAttempt> for ExamAttemptResponse {
    fn from(attempt: ExamAttempt) -> Self {
        let status = if attempt.submitted_at.is_some() {
            "completed".to_string()
        } else if attempt.started_at.is_some() {
            "in_progress".to_string()
        } else {
            "not_started".to_string()
        };

        Self {
            id: attempt.id,
            user_id: attempt.user_id,
            exam_id: attempt.exam_id,
            started_at: attempt.started_at,
            submitted_at: attempt.submitted_at,
            score_total: attempt.score_total,
            status,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Answer {
    pub id: Uuid,
    pub attempt_id: Uuid,
    pub question_id: Uuid,
    pub answer_text: Option<String>,
    pub is_correct: Option<bool>,
    pub score_awarded: Option<i32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct AnswerResponse {
    pub id: Uuid,
    pub attempt_id: Uuid,
    pub question_id: Uuid,
    pub answer_text: Option<String>,
    pub is_correct: Option<bool>,
    pub score_awarded: Option<i32>,
}

impl From<Answer> for AnswerResponse {
    fn from(answer: Answer) -> Self {
        Self {
            id: answer.id,
            attempt_id: answer.attempt_id,
            question_id: answer.question_id,
            answer_text: answer.answer_text,
            is_correct: answer.is_correct,
            score_awarded: answer.score_awarded,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ExamAttemptWithAnswers {
    pub attempt: ExamAttemptResponse,
    pub answers: Vec<AnswerResponse>,
}

#[derive(Debug, Serialize)]
pub struct ExamAttemptWithAnswersRequest {
    pub attempt_id: Uuid,
    pub user_id: Uuid,
}
