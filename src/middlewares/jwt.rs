use std::env;
use axum::{
    http::{Request, StatusCode}, middleware::Next, response::Response,
    body::Body
};
use jsonwebtoken::{
    decode,
    DecodingKey,
    Validation
};
use crate::utils::jwt::Claims;

pub async fn jwt_validation(
    mut request: Request<Body>,
    next: Next
) -> Result<Response, StatusCode> {
    let secret_key = env::var("JWT_KEY").expect("Gagal membaca environment variable");
    let auth_header = request.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::default()
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(token_data);
    Ok(next.run(request).await)
}