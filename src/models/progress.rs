use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub course_name: String,
    pub course_type: CourseType, // Enum: Course, Exam, Assignment, etc.
    pub progress_percentage: Option<i32>, // 0-100, None for started items
    pub status: ProgressStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_score: Option<i32>,
    pub max_score: Option<i32>,
    pub level: i32, // User level based on experience
    pub experience_points: i32, // XP earned from this activity
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "course_type", rename_all = "lowercase")]
pub enum CourseType {
    Course,
    Exam,
    Assignment,
    Quiz,
    Project,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "progress_status", rename_all = "lowercase")]
pub enum ProgressStatus {
    Started,
    InProgress,
    Completed,
    Failed,
    Enrolled,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserProgressResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub course_name: String,
    pub course_type: CourseType,
    pub progress_percentage: Option<i32>,
    pub status: ProgressStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_score: Option<i32>,
    pub max_score: Option<i32>,
    pub level: i32,
    pub experience_points: i32,
}

impl From<UserProgress> for UserProgressResponse {
    fn from(progress: UserProgress) -> Self {
        Self {
            id: progress.id,
            user_id: progress.user_id,
            course_name: progress.course_name,
            course_type: progress.course_type,
            progress_percentage: progress.progress_percentage,
            status: progress.status,
            started_at: progress.started_at,
            completed_at: progress.completed_at,
            total_score: progress.total_score,
            max_score: progress.max_score,
            level: progress.level,
            experience_points: progress.experience_points,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProgressRequest {
    #[validate(length(min = 1))]
    pub course_name: String,
    pub course_type: CourseType,
    pub status: ProgressStatus,
    pub total_score: Option<i32>,
    pub max_score: Option<i32>,
    pub experience_points: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProgressRequest {
    pub progress_percentage: Option<i32>,
    pub status: ProgressStatus,
    pub total_score: Option<i32>,
    pub completed_at: Option<DateTime<Utc>>,
    pub experience_points: Option<i32>,
}

// User Level System
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserLevel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub current_level: i32,
    pub total_experience: i32,
    pub experience_to_next_level: i32,
    pub level_title: String, // Beginner, Intermediate, Advanced, Expert
    pub achievements: Vec<String>, // JSON array of achievement names
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserLevelResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub current_level: i32,
    pub total_experience: i32,
    pub experience_to_next_level: i32,
    pub level_title: String,
    pub achievements: Vec<String>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserLevel> for UserLevelResponse {
    fn from(level: UserLevel) -> Self {
        Self {
            id: level.id,
            user_id: level.user_id,
            current_level: level.current_level,
            total_experience: level.total_experience,
            experience_to_next_level: level.experience_to_next_level,
            level_title: level.level_title,
            achievements: level.achievements,
            updated_at: level.updated_at,
        }
    }
}

// Progress Summary for Dashboard
#[derive(Debug, Serialize)]
pub struct ProgressSummary {
    pub user_level: UserLevelResponse,
    pub recent_activities: Vec<UserProgressResponse>,
    pub completed_courses: i32,
    pub courses_in_progress: i32,
    pub total_experience_earned: i32,
    pub achievements_unlocked: i32,
}

// Achievement System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub experience_reward: i32,
    pub unlock_condition: AchievementCondition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementCondition {
    CompleteExams(i32),        // Complete X exams
    ScoreAbove(i32),          // Score above X%
    ConsecutiveDays(i32),     // Study X consecutive days
    TotalExperience(i32),     // Reach X total experience
    CompleteInTime(i32),      // Complete exam within X minutes
}

impl Achievement {
    pub fn get_default_achievements() -> Vec<Achievement> {
        vec![
            Achievement {
                name: "First Steps".to_string(),
                description: "Complete your first exam".to_string(),
                icon: "🎯".to_string(),
                experience_reward: 100,
                unlock_condition: AchievementCondition::CompleteExams(1),
            },
            Achievement {
                name: "High Achiever".to_string(),
                description: "Score above 90% in an exam".to_string(),
                icon: "🏆".to_string(),
                experience_reward: 200,
                unlock_condition: AchievementCondition::ScoreAbove(90),
            },
            Achievement {
                name: "Dedicated Learner".to_string(),
                description: "Study for 7 consecutive days".to_string(),
                icon: "📚".to_string(),
                experience_reward: 300,
                unlock_condition: AchievementCondition::ConsecutiveDays(7),
            },
            Achievement {
                name: "Experience Master".to_string(),
                description: "Reach 1000 total experience points".to_string(),
                icon: "💎".to_string(),
                experience_reward: 500,
                unlock_condition: AchievementCondition::TotalExperience(1000),
            },
            Achievement {
                name: "Speed Runner".to_string(),
                description: "Complete an exam in under 30 minutes".to_string(),
                icon: "⚡".to_string(),
                experience_reward: 250,
                unlock_condition: AchievementCondition::CompleteInTime(30),
            },
        ]
    }
}
