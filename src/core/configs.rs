use serde::Deserialize;
use config::{Config, ConfigError, Environment};
use once_cell::sync::Lazy;
use tracing_subscriber::{fmt, EnvFilter};
use tracing::{info};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub secret_key: String,
    pub hash_rounds: u32,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok(); // Load `.env` (dev-only)
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

pub fn init_logging() {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();
}



