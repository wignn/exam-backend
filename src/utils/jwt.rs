use crate::errors::{AppError, AppResult};
use crate::models::user::UserRole;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user_id
    pub email: String,
    pub role: UserRole,
    pub exp: i64,
    pub iat: i64,
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_expires_in: i64,
    refresh_expires_in: i64,
}

impl JwtService {
    pub fn new(secret: &str, access_expires_in: i64, refresh_expires_in: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_expires_in,
            refresh_expires_in,
        }
    }

    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        email: &str,
        role: UserRole,
    ) -> AppResult<String> {
        self.generate_token(user_id, email, role, TokenType::Access, self.access_expires_in)
    }

    pub fn generate_refresh_token(
        &self,
        user_id: Uuid,
        email: &str,
        role: UserRole,
    ) -> AppResult<String> {
        self.generate_token(user_id, email, role, TokenType::Refresh, self.refresh_expires_in)
    }

    fn generate_token(
        &self,
        user_id: Uuid,
        email: &str,
        role: UserRole,
        token_type: TokenType,
        expires_in: i64,
    ) -> AppResult<String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(expires_in);

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(AppError::Jwt)
    }

    pub fn verify_token(&self, token: &str) -> AppResult<Claims> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(AppError::Jwt)
    }

    pub fn verify_access_token(&self, token: &str) -> AppResult<Claims> {
        let claims = self.verify_token(token)?;
        match claims.token_type {
            TokenType::Access => Ok(claims),
            TokenType::Refresh => Err(AppError::Unauthorized),
        }
    }

    pub fn verify_refresh_token(&self, token: &str) -> AppResult<Claims> {
        let claims = self.verify_token(token)?;
        match claims.token_type {
            TokenType::Refresh => Ok(claims),
            TokenType::Access => Err(AppError::Unauthorized),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::models::user::UserRole;

    fn setup() -> JwtService {
        let secret = "my_test_secret_key";
        let access_expires_in = 900;      // 15 minutes
        let refresh_expires_in = 604800;  // 7 days
        JwtService::new(secret, access_expires_in, refresh_expires_in)
    }

    #[test]
    fn test_generate_and_verify_access_token() {
        let jwt = setup();
        let user_id = Uuid::new_v4();
        let email = "user@example.com";
        let role = UserRole::Student;

        let token = jwt.generate_access_token(user_id, email, role.clone()).unwrap();
        let claims = jwt.verify_access_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, role);
        assert_eq!(claims.token_type, TokenType::Access);
    }

    #[test]
    fn test_generate_and_verify_refresh_token() {
        let jwt = setup();
        let user_id = Uuid::new_v4();
        let email = "user@example.com";
        let role = UserRole::Admin;

        let token = jwt.generate_refresh_token(user_id, email, role.clone()).unwrap();
        let claims = jwt.verify_refresh_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, role);
        assert_eq!(claims.token_type, TokenType::Refresh);
    }

    #[test]
    fn test_verify_access_token_with_refresh_token_should_fail() {
        let jwt = setup();
        let user_id = Uuid::new_v4();
        let email = "user@example.com";

        let token = jwt.generate_refresh_token(user_id, email, UserRole::Admin).unwrap();
        let result = jwt.verify_access_token(&token);

        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), "Unauthorized");
    }

    #[test]
    fn test_verify_refresh_token_with_access_token_should_fail() {
        let jwt = setup();
        let user_id = Uuid::new_v4();
        let email = "user@example.com";

        let token = jwt.generate_access_token(user_id, email, UserRole::Admin).unwrap();
        let result = jwt.verify_refresh_token(&token);

        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), "Unauthorized");
    }
}
