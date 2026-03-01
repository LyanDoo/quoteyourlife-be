use axum::{Extension, response::IntoResponse};
use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
use quoteyourlife_be::{db::{PgPool, get_conn}, models::User};
use tracing::{info, debug, error, warn};
use axum::{
    Json,
    http::StatusCode,
    http::Request,
    body::Body
};
use serde_json::json;
use serde::Deserialize;
use bcrypt::{
    verify
};
use crate::handlers::AppError;
use crate::utils::jwt::create_jwt;
use crate::utils::jwt::{
    Claims,
    verify_jwt as verify_jwt_utils
};

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String
}

pub async fn login(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<LoginData>
) -> impl IntoResponse {
    info!("[POST /auth/login] Received login request");
    debug!("Login attempt for username: {}", payload.username);
    
    let user_name = payload.username;
    let password = payload.password;
    if user_name == "" || password == "" {
        warn!("[POST /auth/login] Login failed: Username/password is empty");
        error!("Username/password tidak boleh kosong!");
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "fail",
                "message": "Username/password tidak boleh kosong!"
            }))
        )
    }

    let _user_name = user_name.clone();
    let result = tokio::task::spawn_blocking(move || -> Result<_, AppError>{
        let mut conn = get_conn(&pool)?;
        use quoteyourlife_be::schema::users::dsl::*;
        let results = users.filter(username.eq(&_user_name)).load::<User>(&mut conn)?;
        Ok(results)
    })
        .await
        .map_err(AppError::AsyncTaskError).unwrap().unwrap();
    
    if result.is_empty() {
        warn!("[POST /auth/login] Login failed: Username not found - {}", user_name);
        error!("Username tidak ditemukan!");
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "fail",
                "message": "Username tidak ditemukan!"
            }))
        )
    }
    info!("[POST /auth/login] User found: {}", &result[0].username);
    debug!("Verifying password for user: {}", &result[0].username);
    
    if verify(&password, &result[0].password_hash).expect("Gagal verifikasi") {
        info!("[POST /auth/login] Login successful for user: {}", &result[0].username);
        debug!("Generating JWT token for user: {}", &result[0].id);

        let token = tokio::task::spawn_blocking(move || -> Result <String, AppError>{
            create_jwt(&result[0].id.to_string())
        }).await
        .expect("Error pada saat tokenisasi");

        let token = match token {
            Ok(token) => {
                info!("[POST /auth/login] JWT token generated successfully for user: {}", user_name);
                (
                    StatusCode::ACCEPTED,
                    Json(json!({
                        "status": "success",
                        "message": "Login Berhasil",
                        "token": &token
                    }))
                )
            },
            Err(err) => {
                error!("[POST /auth/login] JWT generation failed: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "fail",
                        "message": "Login Gagal, Internal server error"
                    }))
                )
            }
        };
        token

    } else {
        warn!("[POST /auth/login] Login failed: Wrong password for user - {}", user_name);
        error!("Login Gagal: Password Salah!");
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "fail",
                "message": "Login Gagal: Password Salah!"
            }))
        )
    }
    
}

pub async fn verify_jwt(
    request: Request<Body>
) -> Result<Json<Claims>, AppError> {
    info!("[POST /auth/verify] Received JWT verification request");
    debug!("Extracting Authorization header");
    
    let auth_header = request.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            warn!("[POST /auth/verify] JWT verification failed: Missing Authorization header");
            AppError::GeneralError("Internal Server Error".to_string())
        })?;

    debug!("Extracting token from Authorization header");
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| {
            warn!("[POST /auth/verify] JWT verification failed: Invalid token format");
            AppError::GeneralError("Internal Server Error".to_string())
        })?;

    debug!("Verifying JWT token");
    let user_claims = verify_jwt_utils(token)
        .map_err(|err| {
            warn!("[POST /auth/verify] JWT verification failed: {:?}", err);
            error!("JWT validation error: {:?}", err);
            AppError::JWTValidationError(err)
        })?;

    info!("[POST /auth/verify] JWT verification successful for user_id: {}", user_claims.sub);
    Ok(Json(user_claims))
}
