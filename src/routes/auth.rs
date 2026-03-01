use axum::{
    routing::{
        get,
        post
    },
    Router
};
use crate::handlers::auth;

pub fn router() -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .route("/verify", get(auth::verify_jwt))
}