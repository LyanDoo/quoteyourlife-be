use tracing::{info};
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
    let quotes = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let mut conn = get_conn(&pool)?; // '?' sekarang berfungsi!
        use quoteyourlife_be::schema::quotes::dsl::*;
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

    info!("Created new quote: {:?}", new_quote);
    Ok(Json(new_quote))
}
