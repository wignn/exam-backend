use chrono::{DateTime, Utc};
use serde::{
    Serialize,
    Deserialize,
};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Class {
    pub id: Uuid,
    pub name: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ClassMember {
    user_id: Uuid,
    class_id: Uuid,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ClassResponse {
    pub id: Uuid,
    pub name: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}


impl From<Class> for ClassResponse {
    fn from(class: Class) -> Self {
        Self {
            id: class.id,
            name: class.name,
            created_by: class.created_by,
            created_at: class.created_at,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize,Validate)]
pub struct CreateClassRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    pub created_by: Uuid,
}

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct UpdateClassRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
}

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct CreateClassMemberRequest {
    user_id: Uuid,
    class_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateClassMemberRequest {
    user_id: Option<Uuid>,
    class_id: Option<Uuid>,
}