use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::Multipart;
use diesel::prelude::*;
use serde_json::json;
use crate::models::{NewNFT, NewQuote, Quote, NFT};
use crate::db::{PgPool, get_conn}; 
use tracing::{info, error};
use tokio::fs;
use std::path::Path;

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
            .get_result(&mut conn)?; 
        Ok(result)
    })
    .await
    .map_err(AppError::AsyncTaskError)?
    ?;

    info!("Created new quote: {:?}", new_quote);
    Ok(Json(new_quote))
}

pub async fn get_all_nft(
    Extension(pool): Extension<PgPool>
) -> Result<Json<Vec<NFT>>, AppError> {
    let nfts = tokio::task::spawn_blocking(move || -> Result<_, AppError>{
        let mut conn = get_conn(&pool)?;
        use crate::schema::nft::dsl::*;
        let results = nft.load::<NFT>(&mut conn)?;

        Ok(results)
    })
    .await
    .map_err(AppError::AsyncTaskError)?
    ?;

    info!("Fetched {} NFT", nfts.len());
    Ok(Json(nfts))
}

pub async fn create_new_nft(
    Extension(pool): Extension<PgPool>,
    mut multipart: Multipart
) -> Result<Json<NFT>, AppError> {
    let mut author_: String = String::new();
    let mut title_: String = String::new();
    let mut description_: String = String::new();
    let mut filename_: String = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        // println!("debug\n {:?}",field);
        let name = field.name().unwrap().to_string();
        match name.as_str() {
            "author" => {
                author_ = field.text().await.unwrap();
            }
            "title" => {
                title_ = field.text().await.unwrap();
            }
            "description" => {
                description_ = field.text().await.unwrap();
            }
            "image" => {
                filename_ = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap();
                let upload_dir = Path::new("uploads");
                if !upload_dir.exists() {
                    fs::create_dir_all(upload_dir).await.unwrap();
                }
                let file_path = upload_dir.join(&filename_);
                fs::write(&file_path, &data).await.unwrap();
                info!("Bytes successfully written")
            }
            _ => {}
        }
    }

    let new_nft = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let payload = NewNFT {
            title: title_,
            description: description_,
            author: author_,
            filename: filename_,
        };
        let mut conn = get_conn(&pool)?;
        use crate::schema::nft::dsl::*;
        let result = diesel::insert_into(nft)
            .values(&payload)
            .returning(NFT::as_returning())
            .get_result(&mut conn)?;
        Ok(result)
    })
    .await
    .map_err(AppError::AsyncTaskError)?
    ?;

    info!("Created new NFT: {:?}", new_nft);
    Ok(Json(new_nft))
} 
