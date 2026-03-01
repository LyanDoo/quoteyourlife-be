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

    tracing::info!("🚀 Starting QuoteYourLife Backend Server...");
    tracing::debug!("Initializing environment variables");
    
    // Buat pool koneksi database
    tracing::info!("📦 Establishing database connection pool");
    let pool = db::establish_connection();
    tracing::info!("✅ Database connection pool established");

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    tracing::info!("🔧 CORS middleware configured");

    // Buat router Axum
    tracing::info!("🛣️  Setting up API routes");
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
    
    tracing::info!("✅ All routes configured successfully");
    
    // Definisikan alamat server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("🌐 Server is binding to 0.0.0.0:3000");
    tracing::info!("✨ Server running on http://0.0.0.0:3000");
    tracing::info!("📊 Health check available at http://0.0.0.0:3000/health");
    tracing::info!("📚 API Documentation: Check your Postman collection");
    tracing::info!("✅ QuoteYourLife Backend is ready to accept requests!\n");

    // Jalankan server
    axum::serve(listener, app)
        .await
        .unwrap();
}
