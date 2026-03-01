use tracing::{info, debug};
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
    info!("[GET /gallery] Received request to fetch all NFT");
    debug!("Starting database query for NFT");
    
    let nfts = tokio::task::spawn_blocking(move || -> Result<_, AppError>{
        let mut conn = get_conn(&pool)?;
        use quoteyourlife_be::schema::nft::dsl::*;
        let results = nft.load::<NFT>(&mut conn)?;

        Ok(results)
    })
    .await
    .map_err(AppError::AsyncTaskError)?
    ?;

    info!("[GET /gallery] Successfully fetched {} NFT items", nfts.len());
    debug!("Response payload size: {} items", nfts.len());
    Ok(Json(nfts))
}

pub async fn create_new_nft(
    Extension(pool): Extension<PgPool>,
    mut multipart: Multipart
) -> Result<Json<NFT>, AppError> {
    info!("[POST /gallery] Received request to create new NFT");
    debug!("Starting multipart data processing");
    
    let public_dir = env::var("PUBLIC_DIR").expect("Set the Public Directory in .env");
    let mut author_: String = String::new();
    let mut title_: String = String::new();
    let mut description_: String = String::new();
    let mut filename_: String = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        // println!("debug\n {:?}",field);
        let name = field.name().unwrap().to_string();
        debug!("Processing multipart field: {}", name);
        match name.as_str() {
            "author" => {
                author_ = field.text().await.unwrap();
                debug!("Author extracted: {}", author_);
            }
            "title" => {
                title_ = field.text().await.unwrap();
                debug!("Title extracted: {}", title_);
            }
            "description" => {
                description_ = field.text().await.unwrap();
                debug!("Description extracted: {} chars", description_.len());
            }
            "image" => {
                filename_ = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap();
                debug!("File received: {}, size: {} bytes", filename_, data.len());
                let upload_dir = Path::new(&public_dir);
                if !upload_dir.exists() {
                    fs::create_dir_all(upload_dir).await.unwrap();
                    info!("Created upload directory: {}", public_dir);
                }
                let file_path = upload_dir.join(&filename_);
                fs::write(&file_path, &data).await.unwrap();
                info!("[POST /gallery] File uploaded successfully: {} ({} bytes)", filename_, data.len());
            }
            _ => {
                debug!("Unknown field ignored: {}", name);
            }
        }
    }

    debug!("Multipart processing complete. Creating NFT record...");
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

    info!("[POST /gallery] Successfully created new NFT with ID: {}", new_nft.id);
    debug!("Created NFT: {:?}", new_nft);
    Ok(Json(new_nft))
} 
