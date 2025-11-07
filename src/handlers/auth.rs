use actix_web::{
    error, get, post,
    web::{Data, Json},
    Error, Result,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use tracing::error;
use validator::Validate;

use crate::core::configs::AppState;
use crate::schemas::auth::{LoginRequest, LoginResponse, UserMeResponse};
use crate::services::users::{authenticate_user, get_user_by_id};
use crate::utils::utils::{decode_jwt, generate_jwt};

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("/login")]
pub async fn login(
    data: Data<AppState>,
    payload: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Error> {
    // Validate request
    payload.validate().map_err(|e| {
        error!("Validation error: {}", e);
        error::ErrorBadRequest(format!("Validation error: {}", e))
    })?;

    let login_data = payload.into_inner();

    // Authenticate user
    let user = authenticate_user(&data.db, &login_data.identifier, &login_data.password)
        .await
        .map_err(|e| {
            error!("Authentication error: {}", e);
            error::ErrorUnauthorized("Invalid credentials")
        })?;

    // Generate JWT token
    let token = generate_jwt(user.id, &data.config.secret_key).map_err(|e| {
        error!("JWT generation error: {}", e);
        error::ErrorInternalServerError("Failed to generate token")
    })?;

    Ok(Json(LoginResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        access_token: token,
    }))
}

#[utoipa::path(
    get,
    path = "/users/me",
    responses(
        (status = 200, description = "User information retrieved", body = UserMeResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/me")]
pub async fn get_me(
    data: Data<AppState>,
    auth: BearerAuth,
) -> Result<Json<UserMeResponse>, Error> {
    // Decode JWT token
    let claims = decode_jwt(auth.token(), &data.config.secret_key).map_err(|e| {
        error!("JWT decode error: {}", e);
        error::ErrorUnauthorized("Invalid token")
    })?;

    // Parse user ID from claims
    let user_id: i32 = claims.sub.parse().map_err(|e| {
        error!("Failed to parse user ID: {}", e);
        error::ErrorInternalServerError("Invalid token format")
    })?;

    // Get user from database
    let user = get_user_by_id(&data.db, user_id).await.map_err(|e| {
        error!("Database error: {}", e);
        error::ErrorInternalServerError("Failed to retrieve user")
    })?;

    Ok(Json(UserMeResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        avatar_url: user.avatar_url,
    }))
}
