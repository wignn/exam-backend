use crate::models::exams::{CreateExamRequest, UpdateExamRequest, CreateExamAssignmentRequest, DeleteExamAssignmentRequest};
use crate::models::user::UserRole;
use crate::middleware::auth::AuthUser;
use crate::utils::pagination::Pagination;
use uuid::Uuid;
use validator::Validate;
use chrono::{Utc, Duration};

#[cfg(test)]
mod exam_handler_tests {
    use super::*;

    fn create_test_teacher() -> AuthUser {
        AuthUser {
            id: Uuid::new_v4(),
            email: "teacher@test.com".to_string(),
            role: UserRole::Teacher,
        }
    }

    fn create_test_student() -> AuthUser {
        AuthUser {
            id: Uuid::new_v4(),
            email: "student@test.com".to_string(),
            role: UserRole::Student,
        }
    }

    #[tokio::test]
    async fn test_create_exam_request_validation() {
        let now = Utc::now();
        let valid_request = CreateExamRequest {
            title: "Algorithm and Data Structure Midterm".to_string(),
            description: "Comprehensive midterm exam covering algorithms and data structures".to_string(),
            duration_minutes: 120,
            start_time: now + Duration::hours(1),
            end_time: now + Duration::hours(3),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateExamRequest {
            title: "".to_string(), // Empty title should fail
            description: "Test description".to_string(),
            duration_minutes: 0, // Zero duration should fail
            start_time: now + Duration::hours(1),
            end_time: now + Duration::hours(3),
        };
        assert!(invalid_request.validate().is_err());
    }

    #[tokio::test]
    async fn test_update_exam_request_validation() {
        let now = Utc::now();
        let valid_request = UpdateExamRequest {
            title: "Updated Exam Title".to_string(),
            description: "Updated description".to_string(),
            is_active: true,
            duration_minutes: 90,
            start_time: now + Duration::hours(2),
            end_time: now + Duration::hours(3),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = UpdateExamRequest {
            title: "".to_string(), // Empty title should fail
            description: "Test".to_string(),
            is_active: false,
            duration_minutes: 0, // Zero duration should fail
            start_time: now + Duration::hours(1),
            end_time: now + Duration::hours(2),
        };
        assert!(invalid_request.validate().is_err());
    }

    #[tokio::test]
    async fn test_exam_assignment_requests() {
        let exam_id = Uuid::new_v4();
        let class_id = Uuid::new_v4();

        let create_assignment = CreateExamAssignmentRequest {
            exam_id,
            class_id,
        };
        assert!(create_assignment.validate().is_ok());
        assert_eq!(create_assignment.exam_id, exam_id);
        assert_eq!(create_assignment.class_id, class_id);

        let delete_assignment = DeleteExamAssignmentRequest {
            exam_id,
            class_id,
        };
        assert_eq!(delete_assignment.exam_id, exam_id);
        assert_eq!(delete_assignment.class_id, class_id);
    }

    #[tokio::test]
    async fn test_auth_user_roles() {
        let teacher = create_test_teacher();
        let student = create_test_student();

        assert_eq!(teacher.role, UserRole::Teacher);
        assert_eq!(student.role, UserRole::Student);
        assert!(matches!(teacher.role, UserRole::Teacher));
        assert!(!matches!(student.role, UserRole::Teacher));
    }

    #[tokio::test]
    async fn test_uuid_generation() {
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        assert_ne!(id1, id2);
        assert!(id1 != Uuid::nil());
        assert!(id2 != Uuid::nil());
    }

    #[tokio::test]
    async fn test_pagination_functionality() {
        let default_pagination = Pagination {
            page: None,
            limit: None,
            skip: None,
        };
        assert_eq!(default_pagination.limit_or_default(20), 20);
        assert_eq!(default_pagination.skip_or_default(), 0);
        assert_eq!(default_pagination.page_or_default(), 1);

        let custom_pagination = Pagination {
            page: Some(2),
            limit: Some(10),
            skip: Some(5),
        };
        assert_eq!(custom_pagination.limit_or_default(20), 10);
        assert_eq!(custom_pagination.skip_or_default(), 5);
        assert_eq!(custom_pagination.page_or_default(), 2);
    }

    #[tokio::test]
    async fn test_exam_time_validation() {
        let now = Utc::now();

        // Valid exam with future start and end times
        let future_exam = CreateExamRequest {
            title: "Future Exam".to_string(),
            description: "This exam starts in the future".to_string(),
            duration_minutes: 60,
            start_time: now + Duration::hours(2),
            end_time: now + Duration::hours(3),
        };
        assert!(future_exam.validate().is_ok());
        assert!(future_exam.start_time < future_exam.end_time);

        // Exam with past start time (might be valid for some use cases)
        let past_exam = CreateExamRequest {
            title: "Past Exam".to_string(),
            description: "This exam started in the past".to_string(),
            duration_minutes: 60,
            start_time: now - Duration::hours(1),
            end_time: now + Duration::hours(1),
        };
        assert!(past_exam.validate().is_ok());
    }

    #[tokio::test]
    async fn test_exam_duration_validation() {
        let now = Utc::now();

        // Very short exam
        let short_exam = CreateExamRequest {
            title: "Quick Quiz".to_string(),
            description: "Very short exam".to_string(),
            duration_minutes: 5,
            start_time: now + Duration::hours(1),
            end_time: now + Duration::hours(2),
        };
        assert!(short_exam.validate().is_ok());
        assert_eq!(short_exam.duration_minutes, 5);

        // Long exam
        let long_exam = CreateExamRequest {
            title: "Comprehensive Exam".to_string(),
            description: "Very long comprehensive exam".to_string(),
            duration_minutes: 240, // 4 hours
            start_time: now + Duration::hours(1),
            end_time: now + Duration::hours(5),
        };
        assert!(long_exam.validate().is_ok());
        assert_eq!(long_exam.duration_minutes, 240);
    }

    #[tokio::test]
    async fn test_exam_title_and_description_edge_cases() {
        let now = Utc::now();

        // Exam with long title and description
        let with_long_content = CreateExamRequest {
            title: "Very Long Exam Title That Contains Many Words And Describes The Exam Content".to_string(),
            description: "This is a very long description that provides detailed information about the exam content, objectives, and requirements for students taking this comprehensive examination.".to_string(),
            duration_minutes: 90,
            start_time: now + Duration::hours(1),
            end_time: now + Duration::hours(2),
        };
        assert!(with_long_content.validate().is_ok());
        assert!(!with_long_content.title.is_empty());
        assert!(!with_long_content.description.is_empty());

        // Exam with minimal content
        let minimal_exam = CreateExamRequest {
            title: "Q".to_string(), // Single character title
            description: "T".to_string(), // Single character description
            duration_minutes: 15,
            start_time: now + Duration::hours(1),
            end_time: now + Duration::hours(2),
        };
        assert!(minimal_exam.validate().is_ok());
    }

    #[tokio::test]
    async fn test_exam_assignment_business_logic() {
        let teacher = create_test_teacher();
        let student = create_test_student();

        // Only teachers should be able to assign exams to classes
        assert!(matches!(teacher.role, UserRole::Teacher));
        assert!(!matches!(student.role, UserRole::Teacher));

        let exam_id = Uuid::new_v4();
        let class_id = Uuid::new_v4();

        let assignment = CreateExamAssignmentRequest {
            exam_id,
            class_id,
        };

        // Assignment should have valid UUIDs
        assert!(assignment.exam_id != Uuid::nil());
        assert!(assignment.class_id != Uuid::nil());
    }

    #[tokio::test]
    async fn test_exam_status_logic() {
        let now = Utc::now();

        let active_exam = UpdateExamRequest {
            title: "Active Exam".to_string(),
            description: "This exam is active".to_string(),
            is_active: true,
            duration_minutes: 60,
            start_time: now + Duration::hours(1),
            end_time: now + Duration::hours(2),
        };
        assert_eq!(active_exam.is_active, true);

        let inactive_exam = UpdateExamRequest {
            title: "Inactive Exam".to_string(),
            description: "This exam is inactive".to_string(),
            is_active: false,
            duration_minutes: 60,
            start_time: now + Duration::hours(1),
            end_time: now + Duration::hours(2),
        };
        assert_eq!(inactive_exam.is_active, false);
    }
}
