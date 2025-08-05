use crate::database::Database;
use crate::errors::{AppError, AppResult};
use crate::models::question::{
    BulkCreateQuestionsRequest, Choice, ChoiceForStudentResponse, ChoiceResponse,
    CreateQuestionRequest, Question, QuestionForStudentResponse, QuestionResponse, QuestionType, UpdateQuestionRequest,
};
use chrono::Utc;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

pub struct QuestionService {
    db: Database,
}

impl QuestionService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Create a single question with choices
    pub async fn create_question(
        &self,
        exam_id: Uuid,
        request: CreateQuestionRequest,
    ) -> AppResult<QuestionResponse> {
        // Start transaction
        let mut tx = self.db.pool.begin().await?;

        // Insert question
        let question_row = sqlx::query(
            r#"
            INSERT INTO questions (exam_id, question_text, question_type, correct_answer, score)
            VALUES ($1, $2, $3::question_type, $4, $5)
            RETURNING id, exam_id, question_text, question_type::text as question_type, correct_answer, score
            "#
        )
        .bind(&exam_id)
        .bind(&request.question_text)
        .bind(&request.question_type.to_string())
        .bind(&request.correct_answer)
        .bind(&request.score)
        .fetch_one(&mut *tx)
        .await?;

        let question = self.row_to_question(question_row)?;

        // Insert choices if provided
        let mut choices = Vec::new();
        if let Some(choice_requests) = request.choices {
            for choice_request in choice_requests {
                let choice_row = sqlx::query(
                    r#"
                    INSERT INTO choices (question_id, choice_text, is_correct, created_at)
                    VALUES ($1, $2, $3, $4)
                    RETURNING id, question_id, choice_text, is_correct, created_at
                    "#
                )
                .bind(question.id)
                .bind(&choice_request.choice_text)
                .bind(choice_request.is_correct)
                .bind(Utc::now())
                .fetch_one(&mut *tx)
                .await?;

                let choice = self.row_to_choice(choice_row)?;
                choices.push(choice.into());
            }
        }

        tx.commit().await?;

        Ok(QuestionResponse {
            id: question.id,
            exam_id: question.exam_id,
            question_text: question.question_text,
            question_type: question.question_type.parse().unwrap_or(QuestionType::Essay),
            correct_answer: question.correct_answer,
            score: question.score,
            choices,
        })
    }

    /// Get all questions for an exam (for teachers/admins - includes correct answers)
    pub async fn get_questions_by_exam(&self, exam_id: Uuid) -> AppResult<Vec<QuestionResponse>> {
        let question_rows = sqlx::query(
            r#"
            SELECT id, exam_id, question_text, question_type::text as question_type, correct_answer, score
            FROM questions
            WHERE exam_id = $1
            ORDER BY id
            "#
        )
        .bind(exam_id)
        .fetch_all(&self.db.pool)
        .await?;

        let mut questions = Vec::new();
        for question_row in question_rows {
            let question = self.row_to_question(question_row)?;
            
            // Get choices for this question
            let choice_rows = sqlx::query(
                r#"
                SELECT id, question_id, choice_text, is_correct, created_at
                FROM choices
                WHERE question_id = $1
                ORDER BY id
                "#
            )
            .bind(question.id)
            .fetch_all(&self.db.pool)
            .await?;

            let choices: Vec<ChoiceResponse> = choice_rows
                .into_iter()
                .map(|row| {
                    let choice = self.row_to_choice(row)?;
                    Ok(choice.into())
                })
                .collect::<AppResult<Vec<_>>>()?;

            questions.push(QuestionResponse {
                id: question.id,
                exam_id: question.exam_id,
                question_text: question.question_text,
                question_type: question.question_type.parse().unwrap_or(QuestionType::Essay),
                correct_answer: question.correct_answer,
                score: question.score,
                choices,
            });
        }

        Ok(questions)
    }

    /// Get questions for students (without correct answers)
    pub async fn get_questions_for_student(&self, exam_id: Uuid) -> AppResult<Vec<QuestionForStudentResponse>> {
        let question_rows = sqlx::query(
            r#"
            SELECT id, exam_id, question_text, question_type::text as question_type, score
            FROM questions
            WHERE exam_id = $1
            ORDER BY id
            "#
        )
        .bind(exam_id)
        .fetch_all(&self.db.pool)
        .await?;

        let mut questions = Vec::new();
        for question_row in question_rows {
            let question_id: Uuid = question_row.get("id");
            let exam_id: Uuid = question_row.get("exam_id");
            let question_text: String = question_row.get("question_text");
            let question_type: String = question_row.get("question_type");
            let score: i32 = question_row.get("score");

            // Get choices for this question (without is_correct field)
            let choice_rows = sqlx::query(
                r#"
                SELECT id, choice_text
                FROM choices
                WHERE question_id = $1
                ORDER BY id
                "#
            )
            .bind(question_id)
            .fetch_all(&self.db.pool)
            .await?;

            let choices: Vec<ChoiceForStudentResponse> = choice_rows
                .into_iter()
                .map(|row| ChoiceForStudentResponse {
                    id: row.get("id"),
                    choice_text: row.get("choice_text"),
                })
                .collect();

            questions.push(QuestionForStudentResponse {
                id: question_id,
                exam_id,
                question_text,
                question_type: question_type.parse().unwrap_or(QuestionType::Essay),
                score,
                choices,
            });
        }

        Ok(questions)
    }

    /// Get a single question by ID
    pub async fn get_question_by_id(&self, question_id: Uuid) -> AppResult<QuestionResponse> {
        let question_row = sqlx::query(
            r#"
            SELECT id, exam_id, question_text, question_type::text as question_type, correct_answer, score
            FROM questions
            WHERE id = $1
            "#
        )
        .bind(question_id)
        .fetch_optional(&self.db.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Question not found".to_string()))?;

        let question = self.row_to_question(question_row)?;

        // Get choices for this question
        let choice_rows = sqlx::query(
            r#"
            SELECT id, question_id, choice_text, is_correct, created_at
            FROM choices
            WHERE question_id = $1
            ORDER BY id
            "#
        )
        .bind(question.id)
        .fetch_all(&self.db.pool)
        .await?;

        let choices: Vec<ChoiceResponse> = choice_rows
            .into_iter()
            .map(|row| {
                let choice = self.row_to_choice(row)?;
                Ok(choice.into())
            })
            .collect::<AppResult<Vec<_>>>()?;

        Ok(QuestionResponse {
            id: question.id,
            exam_id: question.exam_id,
            question_text: question.question_text,
            question_type: question.question_type.parse().unwrap_or(QuestionType::Essay),
            correct_answer: question.correct_answer,
            score: question.score,
            choices,
        })
    }

    /// Update a question and its choices
    pub async fn update_question(
        &self,
        question_id: Uuid,
        request: UpdateQuestionRequest,
    ) -> AppResult<QuestionResponse> {
        // Start transaction
        let mut tx = self.db.pool.begin().await?;

        // Update question
        let question_row = sqlx::query(
            r#"
            UPDATE questions
            SET question_text = $1, question_type = $2::question_type, correct_answer = $3, score = $4
            WHERE id = $5
            RETURNING id, exam_id, question_text, question_type::text as question_type, correct_answer, score
            "#
        )
        .bind(&request.question_text)
        .bind(request.question_type.to_string())
        .bind(&request.correct_answer)
        .bind(request.score)
        .bind(question_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::NotFound("Question not found".to_string()))?;

        let question = self.row_to_question(question_row)?;

        // Update choices if provided
        let mut choices = Vec::new();
        if let Some(choice_requests) = request.choices {
            for choice_request in choice_requests {
                if let Some(delete) = choice_request.delete {
                    if delete && choice_request.id.is_some() {
                        // Delete choice
                        sqlx::query("DELETE FROM choices WHERE id = $1 AND question_id = $2")
                            .bind(choice_request.id.unwrap())
                            .bind(question_id)
                            .execute(&mut *tx)
                            .await?;
                        continue;
                    }
                }

                if let Some(choice_id) = choice_request.id {
                    // Update existing choice
                    let choice_row = sqlx::query(
                        r#"
                        UPDATE choices
                        SET choice_text = $1, is_correct = $2
                        WHERE id = $3 AND question_id = $4
                        RETURNING id, question_id, choice_text, is_correct, created_at
                        "#
                    )
                    .bind(&choice_request.choice_text)
                    .bind(choice_request.is_correct)
                    .bind(choice_id)
                    .bind(question_id)
                    .fetch_optional(&mut *tx)
                    .await?;

                    if let Some(row) = choice_row {
                        let choice = self.row_to_choice(row)?;
                        choices.push(choice.into());
                    }
                } else {
                    // Create new choice
                    let choice_row = sqlx::query(
                        r#"
                        INSERT INTO choices (question_id, choice_text, is_correct, created_at)
                        VALUES ($1, $2, $3, $4)
                        RETURNING id, question_id, choice_text, is_correct, created_at
                        "#
                    )
                    .bind(question_id)
                    .bind(&choice_request.choice_text)
                    .bind(choice_request.is_correct)
                    .bind(Utc::now())
                    .fetch_one(&mut *tx)
                    .await?;

                    let choice = self.row_to_choice(choice_row)?;
                    choices.push(choice.into());
                }
            }
        } else {
            // If no choices provided, get existing ones
            let choice_rows = sqlx::query(
                r#"
                SELECT id, question_id, choice_text, is_correct, created_at
                FROM choices
                WHERE question_id = $1
                ORDER BY id
                "#
            )
            .bind(question_id)
            .fetch_all(&mut *tx)
            .await?;

            choices = choice_rows
                .into_iter()
                .map(|row| {
                    let choice = self.row_to_choice(row)?;
                    Ok(choice.into())
                })
                .collect::<AppResult<Vec<_>>>()?;
        }

        tx.commit().await?;

        Ok(QuestionResponse {
            id: question.id,
            exam_id: question.exam_id,
            question_text: question.question_text,
            question_type: question.question_type.parse().unwrap_or(QuestionType::Essay),
            correct_answer: question.correct_answer,
            score: question.score,
            choices,
        })
    }

    /// Delete a question and its choices
    pub async fn delete_question(&self, question_id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM questions WHERE id = $1")
            .bind(question_id)
            .execute(&self.db.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Question not found".to_string()));
        }

        Ok(())
    }

    /// Bulk create questions for an exam
    pub async fn bulk_create_questions(
        &self,
        request: BulkCreateQuestionsRequest,
    ) -> AppResult<Vec<QuestionResponse>> {
        let mut created_questions = Vec::new();

        for question_request in request.questions {
            let question = self.create_question(request.exam_id, question_request).await?;
            created_questions.push(question);
        }

        Ok(created_questions)
    }

    /// Get total score for an exam
    pub async fn get_exam_total_score(&self, exam_id: Uuid) -> AppResult<i32> {
        let row = sqlx::query(
            "SELECT COALESCE(SUM(score), 0) as total_score FROM questions WHERE exam_id = $1"
        )
        .bind(exam_id)
        .fetch_one(&self.db.pool)
        .await?;

        Ok(row.get("total_score"))
    }

    // Helper methods
    fn row_to_question(&self, row: PgRow) -> AppResult<Question> {
        Ok(Question {
            id: row.get("id"),
            exam_id: row.get("exam_id"),
            question_text: row.get("question_text"),
            question_type: row.get("question_type"),
            correct_answer: row.get("correct_answer"),
            score: row.get("score"),
        })
    }

    fn row_to_choice(&self, row: PgRow) -> AppResult<Choice> {
        Ok(Choice {
            id: row.get("id"),
            question_id: row.get("question_id"),
            choice_text: row.get("choice_text"),
            is_correct: row.get("is_correct"),
            created_at: row.get("created_at"),
        })
    }
}
