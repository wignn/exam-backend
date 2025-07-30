use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_expires_in: i64,   // in seconds
    pub jwt_refresh_expires_in: i64,  // in seconds
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let config = Config {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/exam_db".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-super-secret-jwt-key".to_string()),
            jwt_access_expires_in: std::env::var("JWT_ACCESS_EXPIRES_IN")
                .unwrap_or_else(|_| "900".to_string()) // 15 minutes
                .parse()?,
            jwt_refresh_expires_in: std::env::var("JWT_REFRESH_EXPIRES_IN")
                .unwrap_or_else(|_| "604800".to_string()) // 7 days
                .parse()?,
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
        };

        Ok(config)
    }
}