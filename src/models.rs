use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

// Ini adalah import dari schema.rs yang dihasilkan Diesel
use crate::schema::quotes;

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