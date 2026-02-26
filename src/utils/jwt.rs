use jsonwebtoken::{
    encode,
    Header,
    EncodingKey
};
use std::time::{
    SystemTime,
    UNIX_EPOCH
};
use std::env;
use serde::{
    Deserialize, 
    Serialize
};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize
}

pub fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error>{
    let secret_key = env::var("JWT_KEY").expect("Gagal membaca environment variable");
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
    let claims = Claims {
        sub: user_id.to_owned(),
        iat: now,
        exp: now + (24 * 3600),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref())
    )
}
