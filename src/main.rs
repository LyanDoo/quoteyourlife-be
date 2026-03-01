mod handlers;
mod utils;
mod middlewares;

use axum::{
    Extension, Router, extract::DefaultBodyLimit, middleware::{self}, routing::{get, post}
};

use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_http::{
    cors::CorsLayer,
    limit::RequestBodyLimitLayer
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use quoteyourlife_be::db;

#[tokio::main]
async fn main() {
    dotenv().ok();
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
        .route("/", get(|| async {"Hello, world!"}))
        .route("/health", get(|| async {"Health: Good"}))
        .route("/quotes", get(handlers::quote::get_all_quotes).post(handlers::quote::create_new_quote))
        .route("/gallery", get(handlers::nft::get_all_nft).post(handlers::nft::create_new_nft))
        .route("/users", get(handlers::user::get_all_users).post(handlers::user::create_new_user).layer(middleware::from_fn(middlewares::jwt::jwt_validation)))
        .route("/article", get(handlers::article::get_all_articles).post(handlers::article::create_new_article))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/verify", post(handlers::auth::verify_jwt))
        .fallback(handlers::handle_404)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(50 * 1024 * 1024))
        .layer(cors) // Middleware CORS
        .layer(Extension(pool)); // Tambahkan pool ke layer Axum agar bisa diakses handler
        
    // Definisikan alamat server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running on http://0.0.0.0:3000");

    // Jalankan server
    axum::serve(listener, app)
        .await
        .unwrap();
}
