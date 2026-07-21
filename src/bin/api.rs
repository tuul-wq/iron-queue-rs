use iron_queue_rs::repository::jobs::JobRepository;
use iron_queue_rs::service::jobs::JobService;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use tracing::info;

use iron_queue_rs::api;
use iron_queue_rs::env_config;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let config = env_config::EnvConfig::from_env()?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async_main(config))
}

async fn async_main(config: env_config::EnvConfig) -> Result<(), Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let listener = tokio::net::TcpListener::bind(&config.api_addr).await?;

    let state = api::AppState {
        job_service: JobService::new(JobRepository::new(pool)),
    };

    info!("Server started on {}", config.api_addr);
    axum::serve(listener, api::setup_routes(state)).await?;

    Ok(())
}
