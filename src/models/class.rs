use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
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
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ClassMember {
    pub user_id: Uuid,
    pub class_id: Uuid,
}


impl From<ClassMember> for ClassMemberResponse {
    fn from(classmember: ClassMember) -> Self {
        Self{
            class_id: classmember.class_id,
            user_id: classmember.user_id
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateClassRequest {
    #[validate(length(min = 2, max = 100, message = "Name must be between 2 and 100 characters"))]
    pub name: String,
}

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct UpdateClassRequest {
    #[validate(length(min = 2, max = 100, message = "Name must be between 2 and 100 characters"))]
    pub name: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateClassMemberRequest {
    pub user_id: Uuid,
    pub class_id: Uuid,
}

#[derive(Debug, Validate, Serialize, FromRow)]
pub struct ClassMemberResponse {
    pub user_id: Uuid,
    pub class_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Validate, Deserialize)]
pub struct DeleteClassMemberRequest {
    pub user_id: Uuid,
    pub class_id: Uuid,
}
