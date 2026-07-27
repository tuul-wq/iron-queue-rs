use iron_queue_rs::repository::jobs::JobRepository;
use iron_queue_rs::service::jobs::JobService;
use iron_queue_rs::utils::shutdown_signal;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;

use iron_queue_rs::api;
use iron_queue_rs::env_config;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn")),
        )
        .with_target(false)
        .compact()
        .init();

    let config = env_config::EnvConfig::from_env()?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async_main(config))
}

async fn async_main(config: env_config::EnvConfig) -> Result<(), Box<dyn Error>> {
    let shutdown_token = CancellationToken::new();

    tracing::info!(max_connections = %config.database_connection_pool, "connecting to PostgreSQL");
    let pool = PgPoolOptions::new()
        .max_connections(config.database_connection_pool)
        .connect(&config.database_url)
        .await
        .map_err(|error| {
            tracing::error!(error = %error, "failed to connect to PostgreSQL");
            error
        })?;

    let listener = tokio::net::TcpListener::bind(&config.api_addr)
        .await
        .map_err(|error| {
            tracing::error!(address = %config.api_addr, error = %error, "failed to bind API listener");
            error
        })?;

    let state = api::AppState {
        job_service: JobService::new(JobRepository::new(pool)),
    };

    tracing::info!(address = %config.api_addr, "API server ready");
    axum::serve(listener, api::setup_routes(state))
        .with_graceful_shutdown(shutdown_signal(shutdown_token))
        .await
        .map_err(|error| {
            tracing::error!(error = %error, "API server stopped unexpectedly");
            error
        })?;

    Ok(())
}
