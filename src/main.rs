mod shared;

use anyhow::Result;
use shared::config::Config;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new(config.rust_log.clone().unwrap_or_else(|| "info".to_string())))?
        )
        .init();

    let addr = config.bind_addr();
    tracing::info!(%addr, "starting server");

    Ok(())
}
