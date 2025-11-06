use crate::core::configs::AppState;
use crate::schemas::auth::{SignShow, SignUp};
use crate::services::users::create_user;
use actix_web::{Responder, Result, get, post, web};
use validator::Validate;

#[post("/signup")]
pub async fn signup(
    data: web::Data<AppState>,
    payload: web::Json<SignUp>,
) -> Result<web::Json<SignShow>> {
    payload
        .validate()
        .map_err(|e| actix_web::error::ErrorBadRequest(format!("Validation error: {}", e)))?;
    let signup_data: SignUp = payload.into_inner();

    let user: SignShow = create_user(&data.db, signup_data).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    Ok(web::Json(user))
}

#[post("/login")]
pub async fn login() -> impl Responder {
    "Login endpoint"
}
