use axum::{
    routing::{
        get,
        post
    },
    middleware::{self},
    Router
};
use crate::handlers::nft;
use crate::middlewares;

pub fn router() -> Router {
    Router::new()
        .route("/", post(nft::create_new_nft).layer(middleware::from_fn(middlewares::jwt::jwt_validation)))
        .route("/", get(nft::get_all_nft))
}