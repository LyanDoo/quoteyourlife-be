use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use chrono::NaiveDateTime;

// Ini adalah import dari schema.rs yang dihasilkan Diesel
use crate::schema::{quotes, nft, users, articles};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = quotes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Quote {
    pub id: Uuid,
    pub text: String,
    pub author: String,
    pub created_at: NaiveDateTime, // Gunakan NaiveDateTime untuk TIMESTAMP tanpa timezone
}

// Struct untuk data yang diterima saat membuat quote baru
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = quotes)]
pub struct NewQuote {
    pub text: String,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = nft)]
pub struct NFT {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub author: String,
    pub filename: String,
    pub created_at: NaiveDateTime
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = nft)]
pub struct NewNFT {
    pub title: String,
    pub description: String,
    pub author: String,
    pub filename: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub created_at: NaiveDateTime
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUsers {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: String
}

#[derive(Debug, DbEnum, Serialize, Deserialize, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::ArticleStatus"] 
// #[diesel(sql_type = ArticleStatus)]
pub enum ArticleStatusEnum {
    Draft,  
    Published
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = articles)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content: Value,
    pub status: ArticleStatusEnum,
    pub author_id: Uuid,
    pub published_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = articles)]
pub struct NewArticle {
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content: Value,
    pub status: ArticleStatusEnum,
    pub author_id: Uuid,
}