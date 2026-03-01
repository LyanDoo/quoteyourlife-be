use axum::{
    routing::{
        get,
        post
    },
    middleware::{self},
    Router
};
use crate::handlers::quote;
use crate::middlewares;

pub fn router() -> Router {
    Router::new()
        .route("/", post(quote::create_new_quote).layer(middleware::from_fn(middlewares::jwt::jwt_validation)))
        .route("/", get(quote::get_all_quotes))
}