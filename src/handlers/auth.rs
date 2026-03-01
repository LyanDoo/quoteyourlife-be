use axum::{Extension, response::IntoResponse};
use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
use quoteyourlife_be::{db::{PgPool, get_conn}, models::User};
use tracing::{info,error};
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
    let user_name = payload.username;
    let password = payload.password;
    if user_name == "" || password == "" {
        error!("Username/password tidak boleh kosong!");
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "fail",
                "message": "Username/password tidak boleh kosong!"
            }))
        )
    }

    let result = tokio::task::spawn_blocking(move || -> Result<_, AppError>{
        let mut conn = get_conn(&pool)?;
        use quoteyourlife_be::schema::users::dsl::*;
        let results = users.filter(username.eq(&user_name)).load::<User>(&mut conn)?;
        Ok(results)
    })
        .await
        .map_err(AppError::AsyncTaskError).unwrap().unwrap();
    
    if result.is_empty() {
        error!("Username tidak ditemukan!");
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "fail",
                "message": "Username tidak ditemukan!"
            }))
        )
    }
    info!("Ditemukan data user [{:?}]", &result[0].username);
    if verify(&password, &result[0].password_hash).expect("Gagal verifikasi") {
        info!("Login Berhasil");

        let token = tokio::task::spawn_blocking(move || -> Result <String, AppError>{
            create_jwt(&result[0].id.to_string())
        }).await
        .expect("Error pada saat tokenisasi");

        let token = match token {
            Ok(token) =>(
                StatusCode::ACCEPTED,
                Json(json!({
                    "status": "success",
                    "message": "Login Berhasil",
                    "token": &token
                }))
            ),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "fail",
                    "message": "Login Gagal, Internal server error"
                }))
            )
        };
        token

    } else {
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
    let auth_header = request.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::GeneralError("Internal Server Error".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::GeneralError("Internal Server Error".to_string()))?;

    let user_claims = verify_jwt_utils(token)
        .map_err(|err| AppError::JWTValidationError(err))?;

    Ok(Json(user_claims))
}
