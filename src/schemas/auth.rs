use serde::{self,Deserialize, Serialize};
use validator::Validate;
use crate::utils::utils::{hash_password_field};


#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct SignUp {
    firt_name: String,
    last_name: String,
    #[validate(email)]
    email: String,
    #[validate(url)]
    image_url: String,
    //implement custom validator to hash password here
    #[serde(deserialize_with = "hash_password_field")]
    password:String,
}

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct SignShow {
    username:String,
    firt_name: String,
    last_name: String,
    #[validate(email)]
    email: String,
    #[validate(url)]
    image_url: String,
    //implement custom validator to hash password here
    password:String,
}
