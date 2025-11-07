use config::{Config, ConfigError, Environment, File, FileFormat};
use deadpool_redis::Pool as RedisPool;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use tracing::info;

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
    pub debug: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        // 1. Load .env file (if it exists) into the environment
        dotenv::dotenv().ok();
        info!("Attempting to load configuration...");

        // 2. Build the config
        let builder = Config::builder()
            // 3. Add `Secrets.toml` as a low-priority source.
            //    This is for `cargo shuttle run` (local dev).
            //    We make it `required(false)` so the app doesn't
            //    crash if it's not present (e.g., in Docker).
            .add_source(
                File::new("Secrets.toml", FileFormat::Toml)
                    .required(false)
            )
            // 4. Add environment variables as the highest-priority source.
            //    This reads from the real environment, which includes
            //    vars from:
            //    - `dotenv()` (from step 1)
            //    - Docker (`--env-file` or `-e`)
            //    - Shuttle production (which injects secrets as env vars)
            .add_source(Environment::default());

        // 5. Build and deserialize
        builder.build()?.try_deserialize()
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis_pool: RedisPool,
    pub config: AppConfig,
}
