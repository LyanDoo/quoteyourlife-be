use axum::{
    routing::get,
    Router,
    Extension,
};

use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod models;
mod schema; // Penting untuk Diesel
mod db;
mod handlers;

#[tokio::main]
async fn main() {
    // Inisialisasi logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Buat pool koneksi database
    let pool = db::establish_connection();

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    // Buat router Axum
    let app = Router::new()
        .route("/quotes", get(handlers::get_all_quotes).post(handlers::create_new_quote))
        .layer(cors) // Middleware CORS
        .layer(Extension(pool)); // Tambahkan pool ke layer Axum agar bisa diakses handler

    // Definisikan alamat server
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("Server running on http://127.0.0.1:3000");

    // Jalankan server
    axum::serve(listener, app)
        .await
        .unwrap();
}