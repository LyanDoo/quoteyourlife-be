use axum::{
    routing::{
        get,
        post
    },
    middleware::{self},
    Router
};
use crate::handlers::article;
use crate::middlewares;

pub fn router() -> Router {
    Router::new()
        .route("/", post(article::create_new_article).layer(middleware::from_fn(middlewares::jwt::jwt_validation)))
        .route("/", get(article::get_all_articles))
}