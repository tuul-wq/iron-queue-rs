use iron_queue_rs::repository::dispatch_policy::DispatchPolicyRepository;
use iron_queue_rs::repository::jobs::JobRepository;
use iron_queue_rs::service::dispatch_policy::DispatchPolicyService;
use iron_queue_rs::utils::shutdown_signal;
use iron_queue_rs::worker::WorkerRunner;
use iron_queue_rs::{domain::DispatchPolicy, env_config};
use sqlx::postgres::{PgListener, PgPoolOptions};
use std::error::Error;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;

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
    let cancel_token = CancellationToken::new();

    tracing::info!(max_connections = %config.database_connection_pool, "connecting to PostgreSQL");
    let pool = PgPoolOptions::new()
        .max_connections(config.database_connection_pool)
        .connect(&config.database_url)
        .await
        .map_err(|error| {
            tracing::error!(error = %error, "failed to connect to PostgreSQL");
            error
        })?;

    tracing::info!(max_connections = %config.database_connection_pool, "connecting to PostgreSQL listener");
    let mut listener = PgListener::connect_with(&pool).await.map_err(|error| {
        tracing::error!(error = %error, "failed to connect to PostgreSQL listener");
        error
    })?;

    listener.listen("dispatch_policy_changed").await?;

    let shutdown_token = cancel_token.clone();
    tokio::spawn(shutdown_signal(shutdown_token));

    let job_repo = JobRepository::new(pool.clone());
    let policy_repo = DispatchPolicyRepository::new(pool.clone());

    let initial_policy = DispatchPolicyService::new(policy_repo)
        .get_latest_policy()
        .await?;

    let (sender, receiver) = watch::channel(initial_policy);

    let shutdown_token = cancel_token.clone();
    tokio::spawn(async move {
        run_policy_listener(listener, sender, shutdown_token).await;
    });

    let mut worker = WorkerRunner::new(job_repo, receiver);

    worker.run(&cancel_token).await?;

    Ok(())
}

async fn run_policy_listener(
    mut pg_listener: PgListener,
    sender: watch::Sender<DispatchPolicy>,
    cancel_token: CancellationToken,
) {
    todo!();
}
