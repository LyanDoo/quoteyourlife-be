// Di dalam file src/handlers.rs

use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use diesel::prelude::*;
use serde_json::json;
use crate::models::{Quote, NewQuote};
use crate::db::{PgPool, get_conn}; // Pastikan get_conn diimpor
use tracing::{info, error};

// 1. Tipe Error Kustom
#[derive(Debug)]
pub enum AppError {
    DatabaseError(diesel::result::Error),
    PoolError(r2d2::Error),
    AsyncTaskError(tokio::task::JoinError),
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
pub async fn get_all_quotes(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Quote>>, AppError> {
    let quotes = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let mut conn = get_conn(&pool)?; // '?' sekarang berfungsi!
        use crate::schema::quotes::dsl::*;
        let results = quotes.load::<Quote>(&mut conn)?; // '?' sekarang berfungsi!
        Ok(results)
    })
    .await
    .map_err(AppError::AsyncTaskError)? // Menangani error dari spawn_blocking
    ?; // Menangani error dari dalam closure (AppError)

    info!("Fetched {} quotes", quotes.len());
    Ok(Json(quotes))
}

pub async fn create_new_quote(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewQuote>,
) -> Result<Json<Quote>, AppError> {
    let new_quote = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let mut conn = get_conn(&pool)?; // '?' sekarang berfungsi!
        use crate::schema::quotes::dsl::*;
        let result = diesel::insert_into(quotes)
            .values(&payload)
            .returning(Quote::as_returning())
            .get_result(&mut conn)?; // '?' sekarang berfungsi!
        Ok(result)
    })
    .await
    .map_err(AppError::AsyncTaskError)?
    ?;

    info!("Created new quote: {:?}", new_quote);
    Ok(Json(new_quote))
}