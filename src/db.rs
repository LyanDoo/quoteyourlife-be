use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

pub type PgPool = Arc<Pool<ConnectionManager<PgConnection>>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Ini adalah blocking pool. Axum akan menjalankan operasi Diesel di thread pool terpisah.
    Arc::new(Pool::builder()
        .build(manager)
        .expect("Failed to create database pool"))
}

// Fungsi helper untuk mendapatkan koneksi dari pool.
// Digunakan di handler.
pub fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, r2d2::Error> {
    pool.get()
}