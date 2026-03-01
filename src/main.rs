mod handlers;
mod utils;
mod middlewares;
mod routes;

use axum::{
    Extension, 
    Router, 
    extract::DefaultBodyLimit, 
    routing::get
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
        .nest("/quotes", routes::quote::router())
        .nest("/gallery", routes::nft::router())
        .nest("/users", routes::user::router())
        .nest("/article", routes::article::router())
        .nest("/auth", routes::auth::router())
        .fallback(handlers::handle_404)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(50 * 1024 * 1024))
        .layer(cors) // Middleware CORS
        .layer(Extension(pool)); 
        
    // Definisikan alamat server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running on http://0.0.0.0:3000");

    // Jalankan server
    axum::serve(listener, app)
        .await
        .unwrap();
}
