pub mod routes;
pub mod schemas;
pub mod core;
pub mod utils;
use deadpool_redis::{Config as RedisConfig, Runtime};
use tracing::{info};
use sea_orm::{Database, DatabaseConnection};
use here::core::configs::{AppConfig,AppState};
use actix_web::{HttpServer, App};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();
    info!("Logger initialized.");
    let settings: AppConfig = AppConfig::from_env().expect("Failed to load configuration");

    let redis_cfg: RedisConfig = RedisConfig::from_url(&settings.redis_url);
    let redis_pool= redis_cfg
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis pool");
    info!("Redis connection pool created.");

    let db: DatabaseConnection = Database::connect(format!("{}?mode=rwc", settings.database_url)).await.expect("Failed to Initialise Database connection");
    info!("Database connection established.");
    db.get_schema_registry("crate::entity::*").sync(&db).await.expect("Failed to sync schema registry");
    info!("Database schema synchronized.");

    let app_state = AppState {
        db,
        redis_pool,
        config: settings.clone(),
    };
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(app_state.clone()))     
    }).bind(("0.0.0.0",8080))?
    .run()
    .await

}
