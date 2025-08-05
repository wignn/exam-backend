#[cfg(test)]
mod exam_handler_tests {
    use crate::handlers::exam::ExamHandlers;
    use crate::models::exams::{CreateExamRequest, UpdateExamRequest, CreateExamAssignmentRequest, DeleteExamAssignmentRequest};
    use crate::models::user::UserRole;
    use crate::middleware::auth::AuthUser;
    use crate::utils::pagination::Pagination;
    use chrono::{Utc, Duration};
    use uuid::Uuid;
    use validator::Validate;

    fn create_test_teacher() -> AuthUser {
        AuthUser {
            id: Uuid::new_v4(),
            email: "teacher@example.com".to_string(),
            role: UserRole::Teacher,
        }
    }

    fn create_test_student() -> AuthUser {
        AuthUser {
            id: Uuid::new_v4(),
            email: "student@example.com".to_string(),
            role: UserRole::Student,
        }
    }

    #[tokio::test]
    async fn test_create_exam_request_validation() {
        let now = Utc::now();
        let start_time = now + Duration::hours(1);
        let end_time = start_time + Duration::hours(2);

        let valid_request = CreateExamRequest {
            title: "Midterm Exam".to_string(),
            description: Some("Algorithm and Data Structure Midterm".to_string()),
            created_by: Uuid::new_v4(),
            duration_minutes: 120,
            start_time,
            end_time,
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateExamRequest {
            title: "".to_string(), // Empty title should fail
            description: Some("Test description".to_string()),
            created_by: Uuid::new_v4(),
            duration_minutes: 120,
            start_time,
            end_time,
        };
        assert!(invalid_request.validate().is_err());
    }

    #[tokio::test]
    async fn test_update_exam_request_validation() {
        let now = Utc::now();
        let start_time = now + Duration::hours(1);
        let end_time = start_time + Duration::hours(2);

        let valid_request = UpdateExamRequest {
            title: "Updated Exam Title".to_string(),
            description: Some("Updated description".to_string()),
            duration_minutes: 150,
            start_time,
            end_time,
            is_active: true,
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = UpdateExamRequest {
            title: "".to_string(),
            description: Some("Test".to_string()),
            duration_minutes: 0, // Zero duration should fail
            start_time,
            end_time,
            is_active: true,
        };
        assert!(invalid_request.validate().is_err());
    }

    #[tokio::test]
    async fn test_exam_assignment_requests() {
        let exam_id = Uuid::new_v4();
        let class_id = Uuid::new_v4();

        let create_request = CreateExamAssignmentRequest {
            exam_id,
            class_id,
        };
        assert!(create_request.validate().is_ok());

        let delete_request = DeleteExamAssignmentRequest {
            exam_id,
            class_id,
        };
        assert!(delete_request.exam_id != Uuid::nil());
        assert!(delete_request.class_id != Uuid::nil());
    }

    #[tokio::test]
    async fn test_exam_time_validation() {
        let now = Utc::now();
        let start_time = now + Duration::hours(1);
        let end_time = start_time + Duration::hours(2);

        // Valid time range
        assert!(end_time > start_time);

        // Duration calculation
        let duration = end_time.signed_duration_since(start_time);
        assert_eq!(duration.num_hours(), 2);
        assert_eq!(duration.num_minutes(), 120);
    }

    #[tokio::test]
    async fn test_exam_duration_scenarios() {
        let now = Utc::now();
        let start_time = now + Duration::hours(1);

        // Test different valid durations
        let valid_durations = vec![30, 60, 90, 120, 180, 240];

        for duration in valid_durations {
            let end_time = start_time + Duration::minutes(duration);
            let request = CreateExamRequest {
                title: "Test Exam".to_string(),
                description: Some("Test description".to_string()),
                created_by: Uuid::new_v4(),
                duration_minutes: duration as i32,
                start_time,
                end_time,
            };
            assert!(request.validate().is_ok());
        }
    }

    #[tokio::test]
    async fn test_teacher_student_authorization() {
        let teacher = create_test_teacher();
        let student = create_test_student();

        assert_eq!(teacher.role, UserRole::Teacher);
        assert_eq!(student.role, UserRole::Student);

        // Only teachers should be able to create exams
        assert!(matches!(teacher.role, UserRole::Teacher));
        assert!(!matches!(student.role, UserRole::Teacher));
    }

    #[tokio::test]
    async fn test_exam_pagination() {
        let pagination = Pagination {
            limit: Some(15),
            skip: Some(10),
        };

        assert_eq!(pagination.limit_or_default(20), 15);
        assert_eq!(pagination.skip_or_default(), 10);

        let default_pagination = Pagination {
            limit: None,
            skip: None,
        };
        assert_eq!(default_pagination.limit_or_default(50), 50);
        assert_eq!(default_pagination.skip_or_default(), 0);
    }

    #[tokio::test]
    async fn test_exam_title_edge_cases() {
        let now = Utc::now();
        let start_time = now + Duration::hours(1);
        let end_time = start_time + Duration::hours(2);

        // Test very long title
        let long_title = "a".repeat(200);
        let long_title_request = CreateExamRequest {
            title: long_title,
            description: Some("Test description".to_string()),
            created_by: Uuid::new_v4(),
            duration_minutes: 120,
            start_time,
            end_time,
        };
        assert!(long_title_request.validate().is_ok());

        // Test title with special characters
        let special_title_request = CreateExamRequest {
            title: "Math & Science Final - 2025 (Advanced Level)".to_string(),
            description: Some("Comprehensive exam".to_string()),
            created_by: Uuid::new_v4(),
            duration_minutes: 180,
            start_time,
            end_time,
        };
        assert!(special_title_request.validate().is_ok());
    }

    #[tokio::test]
    async fn test_exam_time_edge_cases() {
        let now = Utc::now();

        // Test exam starting in the past (should be handled by business logic)
        let past_start = now - Duration::hours(1);
        let past_end = past_start + Duration::hours(2);

        let past_exam_request = CreateExamRequest {
            title: "Past Exam".to_string(),
            description: Some("This exam starts in the past".to_string()),
            created_by: Uuid::new_v4(),
            duration_minutes: 120,
            start_time: past_start,
            end_time: past_end,
        };
        // Validation might pass, but business logic should reject
        assert!(past_exam_request.validate().is_ok());
        assert!(past_start < now); // Verify it's actually in the past

        // Test very short exam duration
        let short_start = now + Duration::hours(1);
        let short_end = short_start + Duration::minutes(5);

        let short_exam_request = CreateExamRequest {
            title: "Quick Quiz".to_string(),
            description: Some("Very short exam".to_string()),
            created_by: Uuid::new_v4(),
            duration_minutes: 5,
            start_time: short_start,
            end_time: short_end,
        };
        assert!(short_exam_request.validate().is_ok());
    }

    #[tokio::test]
    async fn test_uuid_uniqueness_for_exams() {
        let exam_id1 = Uuid::new_v4();
        let exam_id2 = Uuid::new_v4();
        let class_id1 = Uuid::new_v4();
        let class_id2 = Uuid::new_v4();

        // All UUIDs should be unique
        assert_ne!(exam_id1, exam_id2);
        assert_ne!(class_id1, class_id2);
        assert_ne!(exam_id1, class_id1);

        // All UUIDs should not be nil
        assert!(exam_id1 != Uuid::nil());
        assert!(exam_id2 != Uuid::nil());
        assert!(class_id1 != Uuid::nil());
        assert!(class_id2 != Uuid::nil());
    }

    #[tokio::test]
    async fn test_exam_description_handling() {
        let now = Utc::now();
        let start_time = now + Duration::hours(1);
        let end_time = start_time + Duration::hours(2);

        // Test with description
        let with_description = CreateExamRequest {
            title: "Exam with Description".to_string(),
            description: Some("This is a detailed description".to_string()),
            created_by: Uuid::new_v4(),
            duration_minutes: 120,
            start_time,
            end_time,
        };
        assert!(with_description.validate().is_ok());
        assert!(with_description.description.is_some());

        // Test without description
        let without_description = CreateExamRequest {
            title: "Exam without Description".to_string(),
            description: None,
            created_by: Uuid::new_v4(),
            duration_minutes: 120,
            start_time,
            end_time,
        };
        assert!(without_description.validate().is_ok());
        assert!(without_description.description.is_none());
    }
}
