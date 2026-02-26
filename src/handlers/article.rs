use tracing::{info};
use axum::{
    extract::Extension,
    Json,
};
use crate::db::{PgPool, get_conn}; 
use quoteyourlife_be::models::{Article, NewArticle};
use super::AppError;
use diesel::prelude::*;

pub async fn get_all_articles(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Article>>, AppError> {
    let articles = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let mut conn = get_conn(&pool)?;
        use quoteyourlife_be::schema::articles::dsl::*;
        let results = articles.load::<Article>(&mut conn)?;
        Ok(results)
    })
    .await
    .map_err(AppError::AsyncTaskError)?
    ?;

    Ok(Json(articles))
}

pub async fn create_new_article(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewArticle>
) -> Result<Json<Article>, AppError> {
    let new_article = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let mut conn = get_conn(&pool)?;
        use quoteyourlife_be::schema::articles::dsl::*;
        let result = diesel::insert_into(articles)
            .values(payload)
            .returning(Article::as_returning())
            .get_result(&mut conn)?;
        Ok(result)
    })
    .await
    .map_err(AppError::AsyncTaskError)?
    ?;

    info!("Created new article: {:?}", new_article);
    Ok(Json(new_article))
}