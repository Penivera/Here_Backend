// rely on the library crate for modules (declared in src/lib.rs)
// top-level modules are provided by the `here` crate
use actix_web::web::Data;
use actix_web::web::ServiceConfig;
use deadpool_redis::{Config as RedisConfig, Runtime};
use here::core::configs::{AppConfig, AppState};
use here::docs::ApiDoc;
use sea_orm::DatabaseConnection;
use sea_orm::SqlxPostgresConnector;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static> {
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .try_init();

    info!("Logger initialized.");
    let settings: AppConfig = AppConfig::from_env().expect("Failed to load configuration");

    let redis_cfg: RedisConfig = RedisConfig::from_url(&settings.redis_url);
    let redis_pool = redis_cfg
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis pool");
    info!("Redis connection pool created.");

    let db: DatabaseConnection = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
    info!("Database connection established.");

    db.get_schema_registry("here::entity::*")
        .sync(&db)
        .await
        .expect("Failed to sync schema registry");

    info!("Database schema synchronized.");

    let app_state = AppState {
        db,
        redis_pool,
        config: settings.clone(),
    };
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(app_state.clone()));
        // register configured routes from the library crate
        cfg.configure(here::routes::auth::init);

        /// NOTE - Swagger Ui
        cfg.service(SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()));
    };
    Ok(config.into())
}
