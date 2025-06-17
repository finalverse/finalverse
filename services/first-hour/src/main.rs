// services/first-hour/src/main.rs
use first_hour::{FirstHourService, FirstHourConfig};
use tracing::info;
use finalverse_logging as logging;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logging::init(None);

    info!("Starting First Hour Service...");

    let config = FirstHourConfig::from_env();
    let service = FirstHourService::new(config).await?;

    service.run().await?;

    Ok(())
}