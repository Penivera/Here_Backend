use axum::Json;

use crate::schemas::auth::SignUp;

pub async fn sign_up(Json(payload):Json<SignUp>)->Json<SignUp>{Json(payload)}