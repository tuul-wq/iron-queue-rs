use std::error::Error;

use iron_queue_rs::env_config::EnvConfig;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let config = EnvConfig::from_env()?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async_main(config))
}

async fn async_main(config: EnvConfig) -> Result<(), Box<dyn Error>> {
    println!("Hello world");

    Ok(())
}
