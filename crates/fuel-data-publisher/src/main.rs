use clap::Parser;

use fuel_streams_publisher::{
    Publisher,
    FuelCore,
    FuelCoreLike,
};


#[derive(Clone, Parser)]
pub struct Cli {
    #[command(flatten)]
    pub fuel_core_config: fuel_core_bin::cli::run::Command,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let fuel_core: Arc<dyn FuelCoreLike> =
        FuelCore::new(cli.fuel_core_config).await?;

    fuel_core.start().await?;

    let publisher = Publisher::new(
        Arc::clone(&fuel_core),
        telemetry.clone(),
    )
    .await?;

    publisher.run(shutdown_token, historical).await?

    Ok(())
}
