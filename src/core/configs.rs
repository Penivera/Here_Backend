use serde::Deserialize;
use config::{Config, ConfigError, Environment};
use tracing::{info};
use deadpool_redis::Pool as RedisPool;
use sea_orm::DatabaseConnection;

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

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis_pool: RedisPool,
    pub config: AppConfig,
}






