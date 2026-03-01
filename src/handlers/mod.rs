pub mod user;
pub mod nft;
pub mod article;
pub mod quote;
pub mod auth;

use axum::{
    http::StatusCode,
    http::Uri,
    response::{IntoResponse, Response},
    Json,
};

use serde_json::json;
use tracing::{error};



// 1. Tipe Error Kustom
#[derive(Debug)]
pub enum AppError {
    DatabaseError(diesel::result::Error),
    PoolError(r2d2::Error),
    AsyncTaskError(tokio::task::JoinError),
    JWTValidationError(jsonwebtoken::errors::Error),
    GeneralError(String)
}

// 2. Implementasi IntoResponse
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(db_err) => {
                error!("Database error: {:?}", db_err);
                (StatusCode::INTERNAL_SERVER_ERROR, "A database error occurred".to_string())
            }
            AppError::PoolError(pool_err) => {
                error!("Pool connection error: {:?}", pool_err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to connect to the database".to_string())
            }
            AppError::AsyncTaskError(task_err) => {
                error!("Async task error: {:?}", task_err);
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal server error occurred".to_string())
            }
            AppError::JWTValidationError(err) => {
                error!("JWT Error: {:?}", err);
                (StatusCode::UNAUTHORIZED, "JWT Validation Error: Unauthorized".to_string())
            },
            AppError::GeneralError(err) => {
                error!("Error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, err)
            }
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

// 3. Implementasi 'From' untuk konversi error otomatis
impl From<r2d2::Error> for AppError {
    fn from(err: r2d2::Error) -> Self {
        AppError::PoolError(err)
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

// 4. Handler yang sudah diperbaiki

pub async fn handle_404(uri: Uri) -> impl IntoResponse {
    error!("Error '{}' not found", uri);
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "status": "fail",
            "message": "Route not found"
        }))
    )
}
