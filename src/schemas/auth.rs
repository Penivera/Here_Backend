use serde::{self,Deserialize, Serialize};
use validator::Validate;


#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct SignUp {
    firt_name: String,
    last_name: String,
    #[validate(email)]
    email: String,
    #[validate(url)]
    avatar_url: String,
    //implement custom validator to hash password here
    #[validate(length(min = 8))]
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
