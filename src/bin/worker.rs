use iron_queue_rs::env_config;
use iron_queue_rs::repository::jobs::JobRepository;
use iron_queue_rs::worker::{RunnerOutcome, WorkerRunner};
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use tokio::time::{Duration, sleep};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

const POLL_ATTEMPTS: u8 = 3;

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
    let worker = WorkerRunner::new();
    let mut timeout_counter = POLL_ATTEMPTS;

    while timeout_counter > 0 {
        let Ok(outcome) = worker.run_once(&repository).await else {
            sleep(Duration::from_secs(1)).await;
            timeout_counter -= 1;
            continue;
        };

        match outcome {
            RunnerOutcome::Idle => {
                info!(outcome = "idle", "worker run finished");
                sleep(Duration::from_secs(1)).await;
                timeout_counter -= 1;
                continue;
            }
            RunnerOutcome::Completed { job_id } => {
                info!(outcome = "completed", %job_id, "worker run finished");
            }
            RunnerOutcome::Failed { job_id } => {
                info!(outcome = "failed", %job_id, "worker run finished");
            }
        }

        timeout_counter = POLL_ATTEMPTS;
    }

    Ok(())
}
