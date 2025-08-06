use crate::database::Database;
use crate::errors::{AppError, AppResult};
use crate::models::attempt::{
    Answer, AnswerResponse, ExamAttempt, ExamAttemptResponse,
    ExamAttemptWithAnswers, StartExamAttemptRequest, SubmitExamAttemptRequest,
};
use crate::models::progress::{CourseType, CreateProgressRequest, ProgressStatus, UpdateProgressRequest};
use crate::services::progress::ProgressService;
use chrono::Utc;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

pub struct ExamAttemptService {
    db: Database,
}

impl ExamAttemptService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Start a new exam attempt for a user
    pub async fn start_exam_attempt(
        &self,
        request: StartExamAttemptRequest,
        user_id: Uuid,
    ) -> AppResult<ExamAttemptResponse> {
        // Check if exam exists and is active
        let exam_row = sqlx::query(
            "SELECT id, title FROM exams WHERE id = $1 AND is_active = true AND start_time <= NOW() AND end_time >= NOW()"
        )
        .bind(request.exam_id)
        .fetch_optional(&self.db.pool)
        .await?;

        let exam_title = match exam_row {
            Some(row) => row.get::<String, _>("title"),
            None => return Err(AppError::NotFound("Exam not found or not active".to_string())),
        };

        // Check if user has access to this exam (through class assignments)
        let has_access = sqlx::query(
            r#"
            SELECT 1 FROM exam_assignments ea
            JOIN class_members cm ON ea.class_id = cm.class_id
            WHERE ea.exam_id = $1 AND cm.user_id = $2
            "#
        )
        .bind(request.exam_id)
        .bind(user_id)
        .fetch_optional(&self.db.pool)
        .await?;

        if has_access.is_none() {
            return Err(AppError::Forbidden);
        }

        // Check if user already has an attempt for this exam
        let existing_attempt = sqlx::query(
            "SELECT id FROM exam_attempts WHERE exam_id = $1 AND user_id = $2"
        )
        .bind(request.exam_id)
        .bind(user_id)
        .fetch_optional(&self.db.pool)
        .await?;

        if existing_attempt.is_some() {
            return Err(AppError::Conflict("User already has an attempt for this exam".to_string()));
        }

        // Create new exam attempt
        let row = sqlx::query(
            r#"
            INSERT INTO exam_attempts (user_id, exam_id, started_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, exam_id, started_at, submitted_at, score_total
            "#
        )
        .bind(user_id)
        .bind(request.exam_id)
        .bind(Utc::now())
        .fetch_one(&self.db.pool)
        .await?;

        let attempt = self.row_to_exam_attempt(row)?;
        
        // Create progress entry for starting the exam
        if let Err(e) = self.create_exam_progress(user_id, exam_title).await {
            // Log the error but don't fail the exam start
            eprintln!("Failed to create progress entry: {}", e);
        }
        
        Ok(attempt.into())
    }

    /// Submit answers for an exam attempt
    pub async fn submit_exam_attempt(
        &self,
        request: SubmitExamAttemptRequest,
        user_id: Uuid,
    ) -> AppResult<ExamAttemptResponse> {
        // Get the attempt and verify ownership
        let attempt_row = sqlx::query(
            "SELECT id, user_id, exam_id, started_at, submitted_at, score_total FROM exam_attempts WHERE id = $1 AND user_id = $2"
        )
        .bind(request.attempt_id)
        .bind(user_id)
        .fetch_optional(&self.db.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Exam attempt not found".to_string()))?;

        let attempt = self.row_to_exam_attempt(attempt_row)?;

        // Check if already submitted
        if attempt.submitted_at.is_some() {
            return Err(AppError::BadRequest("Exam attempt already submitted".to_string()));
        }

        // Check if exam time has expired
        let exam_row = sqlx::query(
            "SELECT end_time, duration_minutes, title FROM exams WHERE id = $1"
        )
        .bind(attempt.exam_id)
        .fetch_one(&self.db.pool)
        .await?;

        let end_time: chrono::DateTime<Utc> = exam_row.get("end_time");
        let duration_minutes: i32 = exam_row.get("duration_minutes");
        let exam_title: String = exam_row.get("title");
        
        let now = Utc::now();
        let max_end_time = attempt.started_at.unwrap() + chrono::Duration::minutes(duration_minutes as i64);
        
        if now > end_time || now > max_end_time {
            return Err(AppError::BadRequest("Exam time has expired".to_string()));
        }

        // Get maximum possible score for the exam
        let max_score_row = sqlx::query(
            "SELECT COALESCE(SUM(score), 0) as max_score FROM questions WHERE exam_id = $1"
        )
        .bind(attempt.exam_id)
        .fetch_one(&self.db.pool)
        .await?;
        
        let max_score: i32 = max_score_row.get("max_score");

        // Start transaction
        let mut tx = self.db.pool.begin().await?;

        // Insert answers and calculate score
        let mut total_score = 0i32;

        for answer_submission in request.answers {
            // Get question details
            let question_row = sqlx::query(
                "SELECT id, correct_answer, score, question_type FROM questions WHERE id = $1 AND exam_id = $2"
            )
            .bind(answer_submission.question_id)
            .bind(attempt.exam_id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| AppError::NotFound("Question not found".to_string()))?;

            let correct_answer: Option<String> = question_row.get("correct_answer");
            let question_score: i32 = question_row.get("score");
            let question_type: String = question_row.get("question_type");

            // Determine if answer is correct and calculate score
            let (is_correct, score_awarded) = match question_type.as_str() {
                "multiple_choice" | "true_false" => {
                    if let Some(ref correct) = correct_answer {
                        let is_correct = answer_submission.answer_text.trim().eq_ignore_ascii_case(correct.trim());
                        (Some(is_correct), if is_correct { question_score } else { 0 })
                    } else {
                        (None, 0)
                    }
                }
                "essay" => {
                    // Essay questions need manual grading
                    (None, 0)
                }
                _ => (None, 0)
            };

            total_score += score_awarded;

            // Insert answer
            sqlx::query(
                r#"
                INSERT INTO answers (attempt_id, question_id, answer_text, is_correct, score_awarded)
                VALUES ($1, $2, $3, $4, $5)
                "#
            )
            .bind(request.attempt_id)
            .bind(answer_submission.question_id)
            .bind(&answer_submission.answer_text)
            .bind(is_correct)
            .bind(score_awarded)
            .execute(&mut *tx)
            .await?;
        }

        // Update attempt with submission time and score
        let updated_row = sqlx::query(
            r#"
            UPDATE exam_attempts
            SET submitted_at = $1, score_total = $2
            WHERE id = $3
            RETURNING id, user_id, exam_id, started_at, submitted_at, score_total
            "#
        )
        .bind(Utc::now())
        .bind(total_score)
        .bind(request.attempt_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        let updated_attempt = self.row_to_exam_attempt(updated_row)?;
        
        // Update progress entry for completing the exam
        if let Err(e) = self.update_exam_progress(user_id, exam_title, total_score, max_score).await {
            // Log the error but don't fail the exam submission
            eprintln!("Failed to update progress entry: {}", e);
        }
        
        Ok(updated_attempt.into())
    }

    /// Get all attempts for a user
    pub async fn get_user_attempts(&self, user_id: Uuid) -> AppResult<Vec<ExamAttemptResponse>> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, exam_id, started_at, submitted_at, score_total
            FROM exam_attempts
            WHERE user_id = $1
            ORDER BY started_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.db.pool)
        .await?;

        let attempts: Vec<ExamAttemptResponse> = rows
            .into_iter()
            .map(|row| {
                let attempt = self.row_to_exam_attempt(row)?;
                Ok(attempt.into())
            })
            .collect::<AppResult<Vec<_>>>()?;

        Ok(attempts)
    }

    /// Get a specific attempt with answers
    pub async fn get_attempt_with_answers(
        &self,
        attempt_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<ExamAttemptWithAnswers> {
        // Get attempt
        let attempt_row = sqlx::query(
            "SELECT id, user_id, exam_id, started_at, submitted_at, score_total FROM exam_attempts WHERE id = $1 AND user_id = $2"
        )
        .bind(attempt_id)
        .bind(user_id)
        .fetch_optional(&self.db.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Exam attempt not found".to_string()))?;

        let attempt = self.row_to_exam_attempt(attempt_row)?;

        // Get answers
        let answer_rows = sqlx::query(
            r#"
            SELECT id, attempt_id, question_id, answer_text, is_correct, score_awarded
            FROM answers
            WHERE attempt_id = $1
            ORDER BY question_id
            "#
        )
        .bind(attempt_id)
        .fetch_all(&self.db.pool)
        .await?;

        let answers: Vec<AnswerResponse> = answer_rows
            .into_iter()
            .map(|row| {
                let answer = self.row_to_answer(row)?;
                Ok(answer.into())
            })
            .collect::<AppResult<Vec<_>>>()?;

        Ok(ExamAttemptWithAnswers {
            attempt: attempt.into(),
            answers,
        })
    }

    /// Get all attempts for an exam (for teachers/admins)
    pub async fn get_exam_attempts(&self, exam_id: Uuid) -> AppResult<Vec<ExamAttemptResponse>> {
        let rows = sqlx::query(
            r#"
            SELECT ea.id, ea.user_id, ea.exam_id, ea.started_at, ea.submitted_at, ea.score_total
            FROM exam_attempts ea
            WHERE ea.exam_id = $1
            ORDER BY ea.started_at DESC
            "#
        )
        .bind(exam_id)
        .fetch_all(&self.db.pool)
        .await?;

        let attempts: Vec<ExamAttemptResponse> = rows
            .into_iter()
            .map(|row| {
                let attempt = self.row_to_exam_attempt(row)?;
                Ok(attempt.into())
            })
            .collect::<AppResult<Vec<_>>>()?;

        Ok(attempts)
    }

    /// Get current active attempt for user and exam
    pub async fn get_active_attempt(
        &self,
        user_id: Uuid,
        exam_id: Uuid,
    ) -> AppResult<Option<ExamAttemptResponse>> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, exam_id, started_at, submitted_at, score_total
            FROM exam_attempts
            WHERE user_id = $1 AND exam_id = $2 AND submitted_at IS NULL
            "#
        )
        .bind(user_id)
        .bind(exam_id)
        .fetch_optional(&self.db.pool)
        .await?;

        if let Some(row) = row {
            let attempt = self.row_to_exam_attempt(row)?;
            Ok(Some(attempt.into()))
        } else {
            Ok(None)
        }
    }

    // Helper methods
    fn row_to_exam_attempt(&self, row: PgRow) -> AppResult<ExamAttempt> {
        Ok(ExamAttempt {
            id: row.get("id"),
            user_id: row.get("user_id"),
            exam_id: row.get("exam_id"),
            started_at: row.get("started_at"),
            submitted_at: row.get("submitted_at"),
            score_total: row.get("score_total"),
        })
    }

    fn row_to_answer(&self, row: PgRow) -> AppResult<Answer> {
        Ok(Answer {
            id: row.get("id"),
            attempt_id: row.get("attempt_id"),
            question_id: row.get("question_id"),
            answer_text: row.get("answer_text"),
            is_correct: row.get("is_correct"),
            score_awarded: row.get("score_awarded"),
        })
    }

    /// Create progress entry when exam attempt starts
    async fn create_exam_progress(&self, user_id: Uuid, exam_title: String) -> AppResult<()> {
        let progress_service = ProgressService::new(self.db.clone());
        let progress_request = CreateProgressRequest {
            course_name: exam_title,
            course_type: CourseType::Exam,
            status: ProgressStatus::Started,
            total_score: None,
            max_score: None,
            experience_points: Some(0), // No experience for starting
        };

        progress_service.create_progress(user_id, progress_request).await?;
        Ok(())
    }

    /// Update progress when exam attempt is completed
    async fn update_exam_progress(&self, user_id: Uuid, exam_title: String, total_score: i32, max_score: i32) -> AppResult<()> {
        let progress_service = ProgressService::new(self.db.clone());
        
        // Find the progress entry for this exam
        let progress_list = progress_service.get_user_progress(user_id, Some(50)).await?;
        
        if let Some(progress) = progress_list.iter().find(|p| p.course_name == exam_title && p.course_type == CourseType::Exam) {
            let progress_percentage = if max_score > 0 {
                Some((total_score * 100) / max_score)
            } else {
                None
            };

            // Calculate experience based on score
            let experience_points = self.calculate_exam_experience(total_score, max_score);

            let update_request = UpdateProgressRequest {
                progress_percentage,
                status: ProgressStatus::Completed,
                total_score: Some(total_score),
                completed_at: Some(Utc::now()),
                experience_points: Some(experience_points),
            };

            progress_service.update_progress(progress.id, user_id, update_request).await?;
        }
        
        Ok(())
    }

    /// Calculate experience points based on exam performance
    fn calculate_exam_experience(&self, total_score: i32, max_score: i32) -> i32 {
        if max_score == 0 {
            return 50; // Base experience for completion
        }

        let percentage = (total_score * 100) / max_score;
        match percentage {
            90..=100 => 200, // Excellent
            80..=89 => 150,  // Good
            70..=79 => 100,  // Average
            60..=69 => 75,   // Below average
            _ => 50,         // Poor but completed
        }
    }
}
