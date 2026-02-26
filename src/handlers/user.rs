use tracing::{info};
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

    info!("Created new user: {:?}", new_user);
    Ok(Json(new_user))
}

pub async fn get_all_users(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let mut conn = get_conn(&pool)?;
        use quoteyourlife_be::schema::users::dsl::*;
        let results = users.load::<User>(&mut conn)?;
        Ok(results)
    })
    .await
    .map_err(AppError::AsyncTaskError)?
    ?;

    info!("Fetches {} users", users.len());
    Ok(Json(users))
}