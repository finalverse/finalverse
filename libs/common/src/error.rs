use thiserror::Error;
use axum::http::StatusCode;

#[derive(Error, Debug)]
pub enum FinalverseError {
    #[error("Song weaving failed: {0}")]
    SongweavingFailed(String),

    #[error("Echo not found: {0}")]
    EchoNotFound(String),

    #[error("Player not found: {0}")]
    PlayerNotFound(String),

    #[error("Region not found: {0}")]
    RegionNotFound(String),

    #[error("Insufficient resonance: required {required}, available {available}")]
    InsufficientResonance { required: f32, available: f32 },

    #[error("Invalid melody structure: {0}")]
    InvalidMelody(String),

    #[error("Silence corruption detected: {0}")]
    SilenceCorruption(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("UUID parsing error: {0}")]
    UuidError(#[from] uuid::Error),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("AI service error: {0}")]
    AIServiceError(String),

    #[error("Service error: {0}")]
    ServiceError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Resource not available: {0}")]
    ResourceUnavailable(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

// Implement From for StatusCode to make error handling easier
impl From<StatusCode> for FinalverseError {
    fn from(status: StatusCode) -> Self {
        match status {
            StatusCode::BAD_REQUEST => FinalverseError::BadRequest("Bad request".to_string()),
            StatusCode::NOT_FOUND => FinalverseError::ResourceUnavailable("Resource not found".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR => FinalverseError::InternalServerError("Internal server error".to_string()),
            _ => FinalverseError::ServiceError(format!("HTTP error: {}", status)),
        }
    }
}

// Implement IntoResponse for FinalverseError to make it work with Axum
impl axum::response::IntoResponse for FinalverseError {
    fn into_response(self) -> axum::response::Response {
        use axum::response::Json;

        let (status, message) = match self {
            FinalverseError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            FinalverseError::PlayerNotFound(msg) |
            FinalverseError::EchoNotFound(msg) |
            FinalverseError::RegionNotFound(msg) |
            FinalverseError::ResourceUnavailable(msg) => (StatusCode::NOT_FOUND, msg),
            FinalverseError::PermissionDenied(msg) => (StatusCode::FORBIDDEN, msg),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, Json(serde_json::json!({"error": message}))).into_response()
    }
}

pub type Result<T> = std::result::Result<T, FinalverseError>;