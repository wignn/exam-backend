#[cfg(test)]
mod user_service_tests {
    use crate::database::Database;
    use crate::services::user::UserService;
    use crate::models::user::{RegisterRequest, LoginRequest, UserRole};
    use crate::utils::jwt::JwtService;
    use sqlx::PgPool;
    use uuid::Uuid;

    // Helper function to create test database
    async fn create_test_db() -> Database {
        // This would be a test database connection
        // For now, we'll mock the structure
        Database {
            pool: PgPool::connect("postgresql://test:test@localhost/test_db")
                .await
                .expect("Failed to connect to test database"),
        }
    }

    fn create_test_jwt_service() -> JwtService {
        JwtService::new("test_secret", 3600, 86400)
    }

    #[tokio::test]
    #[ignore] // Ignored because it requires a test database
    async fn test_user_registration() {
        let db = create_test_db().await;
        let jwt_service = create_test_jwt_service();
        let user_service = UserService::new(db, jwt_service);

        let register_request = RegisterRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            role: UserRole::Student,
        };

        let result = user_service.register(register_request).await;
        assert!(result.is_ok());

        let auth_response = result.unwrap();
        assert_eq!(auth_response.user.name, "Test User");
        assert_eq!(auth_response.user.email, "test@example.com");
        assert_eq!(auth_response.user.role, UserRole::Student);
        assert!(!auth_response.access_token.is_empty());
        assert!(!auth_response.refresh_token.is_empty());
    }

    #[tokio::test]
    #[ignore] // Ignored because it requires a test database
    async fn test_user_login() {
        let db = create_test_db().await;
        let jwt_service = create_test_jwt_service();
        let user_service = UserService::new(db, jwt_service);

        // First register a user
        let register_request = RegisterRequest {
            name: "Test User".to_string(),
            email: "login@example.com".to_string(),
            password: "password123".to_string(),
            role: UserRole::Student,
        };

        user_service.register(register_request).await.unwrap();

        // Then try to login
        let login_request = LoginRequest {
            email: "login@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = user_service.login(login_request).await;
        assert!(result.is_ok());

        let auth_response = result.unwrap();
        assert_eq!(auth_response.user.email, "login@example.com");
    }

    #[tokio::test]
    #[ignore] // Ignored because it requires a test database
    async fn test_user_login_wrong_password() {
        let db = create_test_db().await;
        let jwt_service = create_test_jwt_service();
        let user_service = UserService::new(db, jwt_service);

        // First register a user
        let register_request = RegisterRequest {
            name: "Test User".to_string(),
            email: "wrongpass@example.com".to_string(),
            password: "password123".to_string(),
            role: UserRole::Student,
        };

        user_service.register(register_request).await.unwrap();

        // Try to login with wrong password
        let login_request = LoginRequest {
            email: "wrongpass@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };

        let result = user_service.login(login_request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_password_validation() {
        // Test password validation logic without database
        use crate::utils::password::PasswordService;

        let password = "test_password123";
        let hashed = PasswordService::hash_password(password).unwrap();
        
        // Verify correct password
        let is_valid = PasswordService::verify_password(password, &hashed).unwrap();
        assert!(is_valid);

        // Verify wrong password
        let is_invalid = PasswordService::verify_password("wrong_password", &hashed).unwrap();
        assert!(!is_invalid);
    }

    #[tokio::test]
    async fn test_jwt_token_generation() {
        let jwt_service = create_test_jwt_service();
        let user_id = Uuid::new_v4();
        let email = "test@example.com";
        let role = UserRole::Student;

        let access_token = jwt_service.generate_access_token(user_id, email, role.clone());
        assert!(access_token.is_ok());
        assert!(!access_token.unwrap().is_empty());

        let refresh_token = jwt_service.generate_refresh_token(user_id, email, role);
        assert!(refresh_token.is_ok());
        assert!(!refresh_token.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_request_validation() {
        use validator::Validate;

        // Test valid register request
        let valid_register = RegisterRequest {
            name: "Valid User".to_string(),
            email: "valid@example.com".to_string(),
            password: "validpass123".to_string(),
            role: UserRole::Student,
        };
        assert!(valid_register.validate().is_ok());

        // Test invalid register request
        let invalid_register = RegisterRequest {
            name: "".to_string(), // Empty name should fail
            email: "invalid_email".to_string(), // Invalid email format
            password: "123".to_string(), // Too short password
            role: UserRole::Student,
        };
        assert!(invalid_register.validate().is_err());

        // Test valid login request
        let valid_login = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        assert!(valid_login.validate().is_ok());

        // Test invalid login request
        let invalid_login = LoginRequest {
            email: "".to_string(), // Empty email
            password: "".to_string(), // Empty password
        };
        assert!(invalid_login.validate().is_err());
    }
}
