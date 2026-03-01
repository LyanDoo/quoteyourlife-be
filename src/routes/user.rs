use axum::{
    routing::{
        get,
        post
    },
    middleware::{self},
    Router,
};
use crate::middlewares;
use crate::handlers::user;

pub fn router() -> Router {
    Router::new()
        .route("/", post(user::create_new_user))
        .route("/", get(user::get_all_users))
        .layer(middleware::from_fn(middlewares::jwt::jwt_validation))
}