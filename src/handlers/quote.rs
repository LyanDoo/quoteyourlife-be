use tracing::{info, debug};
use axum::{
    extract::Extension,
    Json,
};
use crate::db::{PgPool, get_conn}; 
use quoteyourlife_be::models::{Quote, NewQuote};
use super::AppError;
use diesel::prelude::*;

pub async fn get_all_quotes(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Quote>>, AppError> {
    info!("[GET /quotes] Received request to fetch all quotes");
    debug!("Starting database query for quotes");
    
    let quotes = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let mut conn = get_conn(&pool)?; // '?' sekarang berfungsi!
        use quoteyourlife_be::schema::quotes::dsl::*;
        let results = quotes.load::<Quote>(&mut conn)?; // '?' sekarang berfungsi!
        Ok(results)
    })
    .await
    .map_err(AppError::AsyncTaskError)? // Menangani error dari spawn_blocking
    ?; // Menangani error dari dalam closure (AppError)

    info!("[GET /quotes] Successfully fetched {} quotes", quotes.len());
    debug!("Response payload size: {} items", quotes.len());
    Ok(Json(quotes))
}

pub async fn create_new_quote(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewQuote>,
) -> Result<Json<Quote>, AppError> {
    info!("[POST /quotes] Received request to create new quote");
    debug!("Request payload - author: {}, text length: {}", payload.author, payload.text.len());
    
    let new_quote = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let mut conn = get_conn(&pool)?; // '?' sekarang berfungsi!
        use quoteyourlife_be::schema::quotes::dsl::*;
        let result = diesel::insert_into(quotes)
            .values(&payload)
            .returning(Quote::as_returning())
            .get_result(&mut conn)?; 
        Ok(result)
    })
    .await
    .map_err(AppError::AsyncTaskError)?
    ?;

    info!("[POST /quotes] Successfully created new quote with ID: {}", new_quote.id);
    debug!("Created quote: {:?}", new_quote);
    Ok(Json(new_quote))
}
