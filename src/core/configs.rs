use serde::Deserialize;
use config::{Config, ConfigError, Environment};
use once_cell::sync::Lazy;
use tracing::{info};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub secret_key: String,
    pub hash_rounds: u32,
    pub redis_url: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_email: String,
    pub database_url: String,

}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok(); 
        info!("Environment Variables loaded");
        Config::builder()
            .add_source(Environment::default())
            .build()?
            .try_deserialize()
    }

}





pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    AppConfig::from_env().expect("Failed to load configuration")
});




