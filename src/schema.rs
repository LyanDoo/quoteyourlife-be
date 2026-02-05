// @generated automatically by Diesel CLI.

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

diesel::allow_tables_to_appear_in_same_query!(nft, quotes,);
