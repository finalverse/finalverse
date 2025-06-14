mod lib;

// services/first-hour/src/main.rs
use first_hour::{FirstHourService, FirstHourConfig};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    info!("Starting First Hour Service...");

    let config = FirstHourConfig::from_env();
    let mut service = FirstHourService::new(config).await?;

    service.run().await?;

    Ok(())
}