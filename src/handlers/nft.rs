use tracing::{info};
use axum::{
    extract::Extension,
    Json,
};
use crate::db::{PgPool, get_conn}; 
use quoteyourlife_be::models::{NFT, NewNFT};
use super::AppError;
use diesel::prelude::*;
use std::path::Path;
use std::env;
use tokio::fs;
use axum_extra::extract::Multipart;

pub async fn get_all_nft(
    Extension(pool): Extension<PgPool>
) -> Result<Json<Vec<NFT>>, AppError> {
    let nfts = tokio::task::spawn_blocking(move || -> Result<_, AppError>{
        let mut conn = get_conn(&pool)?;
        use quoteyourlife_be::schema::nft::dsl::*;
        let results = nft.load::<NFT>(&mut conn)?;

        Ok(results)
    })
    .await
    .map_err(AppError::AsyncTaskError)?
    ?;

    info!("Fetched {} NFT", nfts.len());
    Ok(Json(nfts))
}

pub async fn create_new_nft(
    Extension(pool): Extension<PgPool>,
    mut multipart: Multipart
) -> Result<Json<NFT>, AppError> {
    let public_dir = env::var("PUBLIC_DIR").expect("Set the Public Directory in .env");
    let mut author_: String = String::new();
    let mut title_: String = String::new();
    let mut description_: String = String::new();
    let mut filename_: String = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        // println!("debug\n {:?}",field);
        let name = field.name().unwrap().to_string();
        match name.as_str() {
            "author" => {
                author_ = field.text().await.unwrap();
            }
            "title" => {
                title_ = field.text().await.unwrap();
            }
            "description" => {
                description_ = field.text().await.unwrap();
            }
            "image" => {
                filename_ = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap();
                let upload_dir = Path::new(&public_dir);
                if !upload_dir.exists() {
                    fs::create_dir_all(upload_dir).await.unwrap();
                }
                let file_path = upload_dir.join(&filename_);
                fs::write(&file_path, &data).await.unwrap();
                info!("Bytes successfully written")
            }
            _ => {}
        }
    }

    let new_nft = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let payload = NewNFT {
            title: title_,
            description: description_,
            author: author_,
            filename: filename_,
        };
        let mut conn = get_conn(&pool)?;
        use quoteyourlife_be::schema::nft::dsl::*;
        let result = diesel::insert_into(nft)
            .values(&payload)
            .returning(NFT::as_returning())
            .get_result(&mut conn)?;
        Ok(result)
    })
    .await
    .map_err(AppError::AsyncTaskError)?
    ?;

    info!("Created new NFT: {:?}", new_nft);
    Ok(Json(new_nft))
} 
