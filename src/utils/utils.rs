#[allow(unused_imports)]
use bcrypt::{hash, verify, BcryptError};
use crate::core::configs::CONFIG;
use serde::{Deserializer,Deserialize};


pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, CONFIG.hash_rounds)
}


pub fn hash_password_field<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    hash_password(&raw).map_err(serde::de::Error::custom)
}
