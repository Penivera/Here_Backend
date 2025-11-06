use std::error::Error;

use crate::entity::prelude::*;
use crate::schemas::auth::{SignShow, SignUp};
use crate::utils::utils::hash_password;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;

pub async fn create_user(
    db: &DatabaseConnection,
    signup: SignUp,
) -> Result<SignShow, Box<dyn Error>> {
    let new_user = UserActiveModel {
        username: Set(signup.username.clone()),
        first_name: Set(signup.first_name.clone()),
        last_name: Set(signup.last_name.clone()),
        email: Set(signup.email.clone()),
        avatar_url: Set(signup.avatar_url.clone()),
        // Password should be hashed before storing
        password: Set(hash_password(&signup.password)),
        ..Default::default()
    };

    let res = User::insert(new_user).exec(db).await?;
    Ok(SignShow {
        id: res.last_insert_id,
        username: signup.username,
        first_name: signup.first_name,
        last_name: signup.last_name,
        email: signup.email,
        avatar_url: signup.avatar_url,
    })
}
