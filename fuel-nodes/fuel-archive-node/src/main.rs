pub mod archive_node;
mod nats_client;

use std::sync::Arc;

use crate::archive_node::ArchiveNode;
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

    ArchiveNode::new(Arc::clone(&fuel_core))
        .await?
        .run()
        .await?;

    Ok(())
}
