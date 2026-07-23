use iron_queue_rs::repository::jobs::JobRepository;
use iron_queue_rs::worker::{RunnerOutcome, WorkerRunner};
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

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
    info!(max_connections = %config.database_connection_pool, "connecting to PostgreSQL");
    let pool = PgPoolOptions::new()
        .max_connections(config.database_connection_pool)
        .connect(&config.database_url)
        .await
        .map_err(|error| {
            error!(error = %error, "failed to connect to PostgreSQL");
            error
        })?;

    let repository = JobRepository::new(pool);

    let outcome = WorkerRunner::new().run_once(&repository).await?;

    match outcome {
        RunnerOutcome::Idle => info!(outcome = "idle", "worker run finished"),
        RunnerOutcome::Completed { job_id } => {
            info!(outcome = "completed", %job_id, "worker run finished")
        }
        RunnerOutcome::Failed { job_id } => {
            info!(outcome = "failed", %job_id, "worker run finished")
        }
    }

    Ok(())
}
