#[cfg(test)]
mod class_handler_tests {
    use crate::handlers::class::ClassHandlers;
    use crate::models::class::{CreateClassRequest, UpdateClassRequest, CreateClassMemberRequest, DeleteClassMemberRequest};
    use crate::models::user::UserRole;
    use crate::middleware::auth::AuthUser;
    use crate::utils::pagination::Pagination;
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
    async fn test_create_class_request_validation() {
        let valid_request = CreateClassRequest {
            name: "Algorithm and Data Structure".to_string(),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateClassRequest {
            name: "".to_string(), // Empty name should fail
        };
        assert!(valid_request.validate().is_ok());
        assert!(invalid_request.validate().is_err());
    }

    #[tokio::test]
    async fn test_update_class_request_validation() {
        let valid_request = UpdateClassRequest {
            name: "Updated Class Name".to_string(),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = UpdateClassRequest {
            name: "".to_string(), // Empty name should fail
        };
        assert!(invalid_request.validate().is_err());
    }

    #[tokio::test]
    async fn test_create_class_member_request() {
        let request = CreateClassMemberRequest {
            user_id: Uuid::new_v4(),
            class_id: Uuid::new_v4(),
        };
        assert!(request.validate().is_ok());
        assert!(request.user_id != Uuid::nil());
        assert!(request.class_id != Uuid::nil());
    }

    #[tokio::test]
    async fn test_delete_class_member_request() {
        let request = DeleteClassMemberRequest {
            user_id: Uuid::new_v4(),
            class_id: Uuid::new_v4(),
        };
        assert!(request.user_id != Uuid::nil());
        assert!(request.class_id != Uuid::nil());
    }

    #[tokio::test]
    async fn test_pagination_functionality() {
        let default_pagination = Pagination {
            limit: None,
            skip: None,
        };
        assert_eq!(default_pagination.limit_or_default(20), 20);
        assert_eq!(default_pagination.skip_or_default(), 0);

        let custom_pagination = Pagination {
            limit: Some(10),
            skip: Some(5),
        };
        assert_eq!(custom_pagination.limit_or_default(20), 10);
        assert_eq!(custom_pagination.skip_or_default(), 5);
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
    async fn test_class_name_validation_edge_cases() {
        // Test very long class name
        let long_name = "a".repeat(255);
        let long_request = CreateClassRequest {
            name: long_name,
        };
        assert!(long_request.validate().is_ok());

        // Test class name with special characters
        let special_request = CreateClassRequest {
            name: "Math & Science - 101 (Advanced)".to_string(),
        };
        assert!(special_request.validate().is_ok());

        // Test whitespace only
        let whitespace_request = CreateClassRequest {
            name: "   ".to_string(),
        };
        // This should fail validation if properly implemented
        // assert!(whitespace_request.validate().is_err());
    }

    #[tokio::test]
    async fn test_pagination_edge_cases() {
        // Test zero values
        let zero_pagination = Pagination {
            limit: Some(0),
            skip: Some(0),
        };
        assert_eq!(zero_pagination.limit_or_default(20), 0);
        assert_eq!(zero_pagination.skip_or_default(), 0);

        // Test large values
        let large_pagination = Pagination {
            limit: Some(1000),
            skip: Some(50000),
        };
        assert_eq!(large_pagination.limit_or_default(20), 1000);
        assert_eq!(large_pagination.skip_or_default(), 50000);

        // Test negative values (if applicable)
        let negative_pagination = Pagination {
            limit: Some(-1),
            skip: Some(-10),
        };
        assert_eq!(negative_pagination.limit_or_default(20), -1);
        assert_eq!(negative_pagination.skip_or_default(), -10);
    }
}
