use axum::{
    extract::Extension,
    Json,
};
use diesel::prelude::*;
use crate::models::{Quote, NewQuote};
use crate::db::{PgPool, get_conn};
use tracing::{info};

// Handler untuk mendapatkan semua quote
pub async fn get_all_quotes(
    Extension(pool): Extension<PgPool>,
) -> Json<Vec<Quote>> {
    let quotes = tokio::task::spawn_blocking(move || {
        let mut conn = get_conn(&pool).expect("Failed to get DB connection");
        // Import skema tabel quotes
        use crate::schema::quotes::dsl::*;
        quotes.load::<Quote>(&mut conn)
            .expect("Error loading quotes")
    })
    .await
    .expect("Failed to run blocking task");

    info!("Fetched {} quotes", quotes.len());
    Json(quotes)
}

// Handler untuk membuat quote baru
pub async fn create_new_quote(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewQuote>,
) -> Json<Quote> {
    let new_quote = tokio::task::spawn_blocking(move || {
        let mut conn = get_conn(&pool).expect("Failed to get DB connection");
        // Import skema tabel quotes
        use crate::schema::quotes::dsl::*;

        diesel::insert_into(quotes)
            .values(&payload)
            .returning(Quote::as_returning())
            .get_result(&mut conn)
            .expect("Error saving new quote")
    })
    .await
    .expect("Failed to run blocking task");

    info!("Created new quote: {:?}", new_quote);
    Json(new_quote)
}