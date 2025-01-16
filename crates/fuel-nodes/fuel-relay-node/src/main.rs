mod nats_client;
mod packets;
pub mod relay_node;
mod subjects;

use std::sync::Arc;

use crate::relay_node::RelayNode;
use clap::Parser;
use fuel_node::{FuelNode, FuelNodeLike};

#[derive(Clone, Parser)]
pub struct Cli {
    #[command(flatten)]
    pub fuel_core_config: fuel_core_bin::cli::run::Command,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let fuel_core: Arc<dyn FuelNodeLike> = FuelNode::new(cli.fuel_core_config).await?;

    let live_publisher = RelayNode::new(Arc::clone(&fuel_core)).await?;

    live_publisher.run().await?;

    Ok(())
}
