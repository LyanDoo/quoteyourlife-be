// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "article_status"))]
    pub struct ArticleStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ArticleStatus;

    articles (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        slug -> Varchar,
        excerpt -> Nullable<Text>,
        content -> Jsonb,
        status -> ArticleStatus,
        author_id -> Uuid,
        published_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    nft (id) {
        id -> Uuid,
        title -> Varchar,
        description -> Varchar,
        author -> Varchar,
        filename -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    quotes (id) {
        id -> Uuid,
        text -> Varchar,
        author -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 100]
        full_name -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(articles -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(articles, nft, quotes, users,);
