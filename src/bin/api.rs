use sqlx::postgres::PgPoolOptions;
use std::error::Error;

use iron_queue_rs::api::routes;
use iron_queue_rs::env_config;

fn main() -> Result<(), Box<dyn Error>> {
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
    axum::serve(listener, routes::setup_routes(pool)).await?;

    Ok(())
}
