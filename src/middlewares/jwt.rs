use std::env;
use axum::{
    http::Request, 
    middleware::Next, 
    response::Response,
    body::Body
};
use jsonwebtoken::{
    decode,
    DecodingKey,
    Validation
};
use crate::{handlers::AppError, utils::jwt::Claims};

pub async fn jwt_validation(
    mut request: Request<Body>,
    next: Next
) -> Result<Response, AppError> {
    let secret_key = env::var("JWT_KEY").expect("Gagal membaca environment variable");
    let auth_header = request.headers()
        .get("Authorization")
        .ok_or(AppError::GeneralError("Authorizaiton header needed!".to_string()))?
        .to_str().unwrap();


    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::GeneralError("Internal Server Error".to_string()))?;

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::default()
    )
    .map_err(|err| AppError::JWTValidationError(err))?;

    request.extensions_mut().insert(token_data);
    Ok(next.run(request).await)
}
