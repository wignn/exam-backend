use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    #[serde(rename = "multiple_choice")]
    MultipleChoice,
    #[serde(rename = "essay")]
    Essay,
    #[serde(rename = "true_false")]
    TrueFalse,
}

impl std::fmt::Display for QuestionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestionType::MultipleChoice => write!(f, "multiple_choice"),
            QuestionType::Essay => write!(f, "essay"),
            QuestionType::TrueFalse => write!(f, "true_false"),
        }
    }
}

impl std::str::FromStr for QuestionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "multiple_choice" => Ok(QuestionType::MultipleChoice),
            "essay" => Ok(QuestionType::Essay),
            "true_false" => Ok(QuestionType::TrueFalse),
            _ => Err(format!("Invalid question type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Question {
    pub id: Uuid,
    pub exam_id: Uuid,
    pub question_text: String,
    pub question_type: String, // Will be converted to/from QuestionType
    pub correct_answer: Option<String>,
    pub score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Choice {
    pub id: Uuid,
    pub question_id: Uuid,
    pub choice_text: String,
    pub is_correct: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateQuestionRequest {
    #[validate(length(min = 1))]
    pub question_text: String,
    pub question_type: QuestionType,
    pub correct_answer: Option<String>,
    #[validate(range(min = 1))]
    pub score: i32,
    pub choices: Option<Vec<CreateChoiceRequest>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateChoiceRequest {
    #[validate(length(min = 1))]
    pub choice_text: String,
    pub is_correct: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateQuestionRequest {
    #[validate(length(min = 1))]
    pub question_text: String,
    pub question_type: QuestionType,
    pub correct_answer: Option<String>,
    #[validate(range(min = 1))]
    pub score: i32,
    pub choices: Option<Vec<UpdateChoiceRequest>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateChoiceRequest {
    pub id: Option<Uuid>, // None for new choices
    #[validate(length(min = 1))]
    pub choice_text: String,
    pub is_correct: bool,
    pub delete: Option<bool>, // True to delete this choice
}

#[derive(Debug, Serialize, Clone)]
pub struct QuestionResponse {
    pub id: Uuid,
    pub exam_id: Uuid,
    pub question_text: String,
    pub question_type: QuestionType,
    pub correct_answer: Option<String>,
    pub score: i32,
    pub choices: Vec<ChoiceResponse>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ChoiceResponse {
    pub id: Uuid,
    pub question_id: Uuid,
    pub choice_text: String,
    pub is_correct: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Choice> for ChoiceResponse {
    fn from(choice: Choice) -> Self {
        Self {
            id: choice.id,
            question_id: choice.question_id,
            choice_text: choice.choice_text,
            is_correct: choice.is_correct,
            created_at: choice.created_at,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct QuestionForStudentResponse {
    pub id: Uuid,
    pub exam_id: Uuid,
    pub question_text: String,
    pub question_type: QuestionType,
    pub score: i32,
    pub choices: Vec<ChoiceForStudentResponse>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ChoiceForStudentResponse {
    pub id: Uuid,
    pub choice_text: String,
}

impl From<Choice> for ChoiceForStudentResponse {
    fn from(choice: Choice) -> Self {
        Self {
            id: choice.id,
            choice_text: choice.choice_text,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BulkCreateQuestionsRequest {
    pub exam_id: Uuid,
    pub questions: Vec<CreateQuestionRequest>,
}
