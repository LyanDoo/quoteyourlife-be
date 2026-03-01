use tracing::{info, debug};
use axum::{
    extract::Extension,
    Json,
};
use crate::db::{PgPool, get_conn}; 
use quoteyourlife_be::models::{User, NewUser};
use super::AppError;
use diesel::prelude::*;



pub async fn create_new_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewUser>
) -> Result<Json<User>, AppError> {
    info!("[POST /users] Received request to create new user");
    debug!("Request payload - username: {}, email: {}", payload.username, payload.email);
    
    let new_user = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let mut conn = get_conn(&pool)?;
        use quoteyourlife_be::schema::users::dsl::*;
        let result = diesel::insert_into(users)
            .values(&payload)
            .returning(User::as_returning())
            .get_result(&mut conn)?;
        Ok(result)
    })
    .await
    .map_err(AppError::AsyncTaskError)?
    ?;

    info!("[POST /users] Successfully created new user with ID: {}", new_user.id);
    debug!("Created user: username={}", new_user.username);
    Ok(Json(new_user))
}

pub async fn get_all_users(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<User>>, AppError> {
    info!("[GET /users] Received request to fetch all users");
    debug!("Starting database query for users");
    
    let users = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let mut conn = get_conn(&pool)?;
        use quoteyourlife_be::schema::users::dsl::*;
        let results = users.load::<User>(&mut conn)?;
        Ok(results)
    })
    .await
    .map_err(AppError::AsyncTaskError)?
    ?;

    info!("[GET /users] Successfully fetched {} users", users.len());
    debug!("Response payload size: {} items", users.len());
    Ok(Json(users))
}