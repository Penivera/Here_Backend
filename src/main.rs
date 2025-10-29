pub mod routes;
pub mod schemas;
pub mod core;
pub mod utils;
use here::core::configs::CONFIG as settings;
use deadpool_redis::{Config as RedisConfig, Runtime};
use tracing::{info};
use sea_orm::{Database, DatabaseConnection};



#[actix_web::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    let redis_cfg = RedisConfig::from_url(&settings.redis_url);
    let redis_pool = redis_cfg
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis pool");
    info!("Redis connection pool created.");

    let db: DatabaseConnection = Database::connect(format!("{}?mode=rwc", settings.database_url)).await.expect("Failed to Initialise Database connection");
    info!("Database connection established.");
    db.get_schema_registry("my_crate::entity::*").sync(&db).await.expect("Failed to sync schema registry");
    info!("Database schema synchronized.");


}
