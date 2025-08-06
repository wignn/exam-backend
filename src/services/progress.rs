use crate::database::Database;
use crate::errors::{AppError, AppResult};
use crate::models::progress::{
    Achievement, AchievementCondition, CourseType, CreateProgressRequest, ProgressStatus,
    ProgressSummary, UpdateProgressRequest, UserLevel, UserLevelResponse, UserProgress,
    UserProgressResponse,
};
use chrono::Utc;
use serde_json;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

pub struct ProgressService {
    db: Database,
}

impl ProgressService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Create a new progress entry
    pub async fn create_progress(
        &self,
        user_id: Uuid,
        request: CreateProgressRequest,
    ) -> AppResult<UserProgressResponse> {
        // Get user's current level for experience calculation
        let user_level = self.get_user_level(user_id).await?;
        let experience_points = request.experience_points.unwrap_or(0);

        let row = sqlx::query(
            r#"
            INSERT INTO user_progress (user_id, course_name, course_type, status, total_score, max_score, level, experience_points, started_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, user_id, course_name, course_type, progress_percentage, status, started_at, completed_at, total_score, max_score, level, experience_points
            "#
        )
        .bind(user_id)
        .bind(&request.course_name)
        .bind(&request.course_type)
        .bind(&request.status)
        .bind(request.total_score)
        .bind(request.max_score)
        .bind(user_level.current_level)
        .bind(experience_points)
        .bind(Utc::now())
        .fetch_one(&self.db.pool)
        .await?;

        let progress = self.row_to_progress(row)?;

        // Update user level if experience was gained
        if experience_points > 0 {
            self.update_user_experience(user_id, experience_points).await?;
        }

        Ok(progress.into())
    }

    /// Update existing progress
    pub async fn update_progress(
        &self,
        progress_id: Uuid,
        user_id: Uuid,
        request: UpdateProgressRequest,
    ) -> AppResult<UserProgressResponse> {
        // Calculate completion status and experience
        let (completed_at, experience_bonus) = match request.status {
            ProgressStatus::Completed => {
                let completion_time = request.completed_at.unwrap_or_else(|| Utc::now());
                let base_exp = request.experience_points.unwrap_or(0);
                let bonus = self.calculate_completion_bonus(&request).await;
                (Some(completion_time), base_exp + bonus)
            }
            _ => (None, request.experience_points.unwrap_or(0)),
        };

        let row = sqlx::query(
            r#"
            UPDATE user_progress 
            SET progress_percentage = $1, status = $2, total_score = $3, completed_at = $4, experience_points = experience_points + $5
            WHERE id = $6 AND user_id = $7
            RETURNING id, user_id, course_name, course_type, progress_percentage, status, started_at, completed_at, total_score, max_score, level, experience_points
            "#
        )
        .bind(request.progress_percentage)
        .bind(&request.status)
        .bind(request.total_score)
        .bind(completed_at)
        .bind(experience_bonus)
        .bind(progress_id)
        .bind(user_id)
        .fetch_one(&self.db.pool)
        .await?;

        let progress = self.row_to_progress(row)?;

        // Update user level if experience was gained
        if experience_bonus > 0 {
            self.update_user_experience(user_id, experience_bonus).await?;
            
            // Check for achievements
            self.check_and_unlock_achievements(user_id, &progress).await?;
        }

        Ok(progress.into())
    }

    /// Get user's progress history
    pub async fn get_user_progress(
        &self,
        user_id: Uuid,
        limit: Option<i32>,
    ) -> AppResult<Vec<UserProgressResponse>> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, course_name, course_type, progress_percentage, status, started_at, completed_at, total_score, max_score, level, experience_points
            FROM user_progress
            WHERE user_id = $1
            ORDER BY started_at DESC
            LIMIT $2
            "#
        )
        .bind(user_id)
        .bind(limit.unwrap_or(50))
        .fetch_all(&self.db.pool)
        .await?;

        let progress_list = rows
            .into_iter()
            .map(|row| {
                let progress = self.row_to_progress(row)?;
                Ok(progress.into())
            })
            .collect::<AppResult<Vec<_>>>()?;

        Ok(progress_list)
    }

    /// Get user level and experience info
    pub async fn get_user_level(&self, user_id: Uuid) -> AppResult<UserLevelResponse> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, current_level, total_experience, experience_to_next_level, level_title, achievements, updated_at
            FROM user_levels
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_optional(&self.db.pool)
        .await?;

        if let Some(row) = row {
            let user_level = self.row_to_user_level(row)?;
            Ok(user_level.into())
        } else {
            // Create initial level for new user
            self.create_initial_user_level(user_id).await
        }
    }

    /// Get progress summary for dashboard
    pub async fn get_progress_summary(&self, user_id: Uuid) -> AppResult<ProgressSummary> {
        let user_level = self.get_user_level(user_id).await?;
        let recent_activities = self.get_user_progress(user_id, Some(10)).await?;
        
        // Get statistics
        let stats_row = sqlx::query(
            r#"
            SELECT 
                COUNT(CASE WHEN status = 'completed' THEN 1 END) as completed_courses,
                COUNT(CASE WHEN status = 'inprogress' OR status = 'started' THEN 1 END) as courses_in_progress,
                COALESCE(SUM(experience_points), 0) as total_experience_earned
            FROM user_progress
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_one(&self.db.pool)
        .await?;

        let completed_courses: i32 = stats_row.get("completed_courses");
        let courses_in_progress: i32 = stats_row.get("courses_in_progress");
        let total_experience_earned: i32 = stats_row.get("total_experience_earned");
        let achievements_unlocked = user_level.achievements.len() as i32;

        Ok(ProgressSummary {
            user_level,
            recent_activities,
            completed_courses,
            courses_in_progress,
            total_experience_earned,
            achievements_unlocked,
        })
    }

    /// Update user experience and level
    async fn update_user_experience(&self, user_id: Uuid, experience_gained: i32) -> AppResult<()> {
        let current_level = self.get_user_level(user_id).await?;
        let new_total_experience = current_level.total_experience + experience_gained;
        
        // Calculate new level based on experience
        let (new_level, level_title) = self.calculate_level_from_experience(new_total_experience);
        let experience_to_next_level = self.calculate_experience_to_next_level(new_level);

        sqlx::query(
            r#"
            UPDATE user_levels 
            SET current_level = $1, total_experience = $2, experience_to_next_level = $3, level_title = $4, updated_at = $5
            WHERE user_id = $6
            "#
        )
        .bind(new_level)
        .bind(new_total_experience)
        .bind(experience_to_next_level)
        .bind(&level_title)
        .bind(Utc::now())
        .bind(user_id)
        .execute(&self.db.pool)
        .await?;

        Ok(())
    }

    /// Create initial level for new user
    async fn create_initial_user_level(&self, user_id: Uuid) -> AppResult<UserLevelResponse> {
        let row = sqlx::query(
            r#"
            INSERT INTO user_levels (user_id, current_level, total_experience, experience_to_next_level, level_title, achievements, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, current_level, total_experience, experience_to_next_level, level_title, achievements, updated_at
            "#
        )
        .bind(user_id)
        .bind(1)  // Starting level
        .bind(0)  // Starting experience
        .bind(100) // Experience needed to reach level 2
        .bind("Beginner")
        .bind(serde_json::to_value(Vec::<String>::new()).unwrap()) // Empty achievements
        .bind(Utc::now())
        .fetch_one(&self.db.pool)
        .await?;

        let user_level = self.row_to_user_level(row)?;
        Ok(user_level.into())
    }

    /// Check and unlock achievements
    async fn check_and_unlock_achievements(&self, user_id: Uuid, progress: &UserProgress) -> AppResult<()> {
        let mut current_level = self.get_user_level(user_id).await?;
        let achievements = Achievement::get_default_achievements();
        let mut new_achievements = Vec::new();

        for achievement in achievements {
            if current_level.achievements.contains(&achievement.name) {
                continue; // Already unlocked
            }

            let should_unlock = match achievement.unlock_condition {
                AchievementCondition::CompleteExams(count) => {
                    self.count_completed_exams(user_id).await? >= count
                }
                AchievementCondition::ScoreAbove(score) => {
                    if let (Some(total), Some(max)) = (progress.total_score, progress.max_score) {
                        (total * 100 / max) >= score
                    } else {
                        false
                    }
                }
                AchievementCondition::TotalExperience(exp) => {
                    current_level.total_experience >= exp
                }
                AchievementCondition::CompleteInTime(_minutes) => {
                    // This would need exam duration tracking
                    false // TODO: Implement when exam duration tracking is added
                }
                AchievementCondition::ConsecutiveDays(_days) => {
                    // TODO: Implement consecutive days tracking
                    false
                }
            };

            if should_unlock {
                new_achievements.push(achievement.name.clone());
                // Award experience for achievement
                self.update_user_experience(user_id, achievement.experience_reward).await?;
            }
        }

        if !new_achievements.is_empty() {
            current_level.achievements.extend(new_achievements);
            
            sqlx::query(
                "UPDATE user_levels SET achievements = $1 WHERE user_id = $2"
            )
            .bind(serde_json::to_value(&current_level.achievements).unwrap())
            .bind(user_id)
            .execute(&self.db.pool)
            .await?;
        }

        Ok(())
    }

    // Helper functions
    async fn calculate_completion_bonus(&self, request: &UpdateProgressRequest) -> i32 {
        let base_bonus = 50; // Base completion bonus
        
        // Score bonus
        let score_bonus = if let Some(total_score) = request.total_score {
            match total_score {
                90..=100 => 50, // Excellent
                80..=89 => 30,  // Good  
                70..=79 => 20,  // Average
                _ => 10,        // Below average
            }
        } else {
            0
        };

        base_bonus + score_bonus
    }

    fn calculate_level_from_experience(&self, total_experience: i32) -> (i32, String) {
        let level = match total_experience {
            0..=99 => 1,
            100..=299 => 2,
            300..=599 => 3,
            600..=999 => 4,
            1000..=1499 => 5,
            1500..=2099 => 6,
            2100..=2799 => 7,
            2800..=3599 => 8,
            3600..=4499 => 9,
            4500..=5499 => 10,
            _ => 10 + (total_experience - 5500) / 1000,
        };

        let title = match level {
            1..=2 => "Beginner",
            3..=5 => "Intermediate", 
            6..=8 => "Advanced",
            9..=12 => "Expert",
            _ => "Master",
        };

        (level, title.to_string())
    }

    fn calculate_experience_to_next_level(&self, current_level: i32) -> i32 {
        match current_level {
            1 => 100,
            2 => 200,
            3 => 300,
            4 => 400,
            5 => 500,
            6 => 600,
            7 => 700,
            8 => 800,
            9 => 900,
            10 => 1000,
            _ => 1000 + (current_level - 10) * 100,
        }
    }

    async fn count_completed_exams(&self, user_id: Uuid) -> AppResult<i32> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM user_progress
            WHERE user_id = $1 AND status = 'completed' AND course_type = $2
            "#
        )
        .bind(user_id)
        .bind(&CourseType::Exam)
        .fetch_one(&self.db.pool)
        .await?;

        Ok(row.get("count"))
    }

    // Row mapping functions
    fn row_to_progress(&self, row: PgRow) -> AppResult<UserProgress> {
        Ok(UserProgress {
            id: row.get("id"),
            user_id: row.get("user_id"),
            course_name: row.get("course_name"),
            course_type: row.get("course_type"),
            progress_percentage: row.get("progress_percentage"),
            status: row.get("status"),
            started_at: row.get("started_at"),
            completed_at: row.get("completed_at"),
            total_score: row.get("total_score"),
            max_score: row.get("max_score"),
            level: row.get("level"),
            experience_points: row.get("experience_points"),
        })
    }

    fn row_to_user_level(&self, row: PgRow) -> AppResult<UserLevel> {
        let achievements_json: serde_json::Value = row.get("achievements");
        let achievements: Vec<String> = serde_json::from_value(achievements_json)
            .map_err(|e| AppError::BadRequest(format!("Failed to parse achievements: {}", e)))?;

        Ok(UserLevel {
            id: row.get("id"),
            user_id: row.get("user_id"),
            current_level: row.get("current_level"),
            total_experience: row.get("total_experience"),
            experience_to_next_level: row.get("experience_to_next_level"),
            level_title: row.get("level_title"),
            achievements,
            updated_at: row.get("updated_at"),
        })
    }
}
