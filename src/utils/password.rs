use crate::errors::{AppError, AppResult};
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct PasswordService;

impl PasswordService {
    pub fn hash_password(password: &str) -> AppResult<String> {
        hash(password, DEFAULT_COST).map_err(AppError::Bcrypt)
    }

    pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
        verify(password, hash).map_err(AppError::Bcrypt)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_success() {
        let password = "test_password123";
        let result = PasswordService::hash_password(password);

        assert!(result.is_ok());
        let hashed = result.unwrap();
        assert_ne!(hashed, password); // Hash should be different from original
        assert!(hashed.len() > 50); // Bcrypt hash should be long
    }

    #[test]
    fn test_hash_password_empty() {
        let password = "";
        let result = PasswordService::hash_password(password);

        assert!(result.is_ok()); // Bcrypt can hash empty strings
    }

    #[test]
    fn test_verify_password_correct() {
        let password = "test_password123";
        let hashed = PasswordService::hash_password(password).unwrap();

        let result = PasswordService::verify_password(password, &hashed);
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should return true for correct password
    }

    #[test]
    fn test_verify_password_incorrect() {
        let password = "test_password123";
        let wrong_password = "wrong_password";
        let hashed = PasswordService::hash_password(password).unwrap();

        let result = PasswordService::verify_password(wrong_password, &hashed);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false for wrong password
    }

    #[test]
    fn test_verify_password_invalid_hash() {
        let password = "test_password123";
        let invalid_hash = "invalid_hash";

        let result = PasswordService::verify_password(password, invalid_hash);
        assert!(result.is_err()); // Should return error for invalid hash
    }

    #[test]
    fn test_hash_consistency() {
        let password = "test_password123";
        let hash1 = PasswordService::hash_password(password).unwrap();
        let hash2 = PasswordService::hash_password(password).unwrap();

        // Hashes should be different (bcrypt uses random salt)
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        assert!(PasswordService::verify_password(password, &hash1).unwrap());
        assert!(PasswordService::verify_password(password, &hash2).unwrap());
    }
}