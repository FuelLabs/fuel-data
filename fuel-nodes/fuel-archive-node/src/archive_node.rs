use std::sync::Arc;
use std::time::Duration;

use fuel_data_subjects::BlocksSubjectFilter;
use fuel_data_subjects::Filter;

use futures::{StreamExt, TryStreamExt};

use crate::nats_client::ArchiveNodeNatsClient;

use fuel_node_publishing::packets::build_block_packet;
use fuel_node_publishing::packets::BuildPacketCommonArgs;

use fuel_core::database::database_description::DatabaseHeight;
use fuel_data_types::*;
use fuel_node::types::*;

use fuel_node::FuelNodeLike;

pub struct ArchiveNode {
    pub fuel_core: Arc<dyn FuelNodeLike>,
}

impl ArchiveNode {
    pub async fn new(fuel_core: Arc<dyn FuelNodeLike>) -> anyhow::Result<Self> {
        fuel_core.start().await?;

        Ok(Self { fuel_core })
    }

    async fn get_last_published_block_height(&self) -> anyhow::Result<u32> {
        Ok(
            ArchiveNodeNatsClient::get_last_published(&BlocksSubjectFilter {
                producer: Filter::All,
                block_height: Filter::All,
            })
            .await?
            .map(|block| block.height)
            .unwrap_or(0),
        )
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let fuel_core = &self.fuel_core;
        let _base_asset_id = Arc::new(*fuel_core.base_asset_id());

        loop {
            let latest_block_height = self.fuel_core.get_latest_block_height()?;
            let last_published_block_height = self.get_last_published_block_height().await?;

            tracing::info!("Latest block height: {:?}", latest_block_height);
            tracing::info!(
                "last_published_block_height: {:?}",
                last_published_block_height
            );

            assert!(last_published_block_height <= latest_block_height);
            futures::stream::iter(last_published_block_height..latest_block_height)
                .then(|block_height| async move {
                    tracing::info!("Publishing for block height: {:?}...", block_height);
                    let sealed_block = fuel_core.get_sealed_block_by_height(block_height);
                    let (block, block_producer) = fuel_core.get_block_and_producer(sealed_block);

                    let block_height = block.header().consensus().height;
                    let transactions = block.transactions();
                    let chain_id = Arc::new(*fuel_core.chain_id());
                    let transaction_ids = transactions
                        .iter()
                        .map(|tx| tx.id(&chain_id).into())
                        .map(|tx_id: [u8; 32]| tx_id.into())
                        .collect::<Vec<Bytes32Proto>>();

                    let consensus: ConsensusProto = fuel_core
                        .get_consensus(&block_height)
                        .expect("Consensus must be found")
                        .into();
                    let block_height = block_height.as_u64() as u32;

                    let build_args = BuildPacketCommonArgs {
                        block_producer: Arc::new(block_producer.into()),
                        block_height,
                        consensus: Arc::new(consensus),
                    };
                    let block_packet = build_block_packet(&block, transaction_ids, build_args);

                    async move {
                        // Publish the block packet to NATS
                        tracing::info!(
                            "About to call NAts client for publsing block height: {:?}...",
                            block_height
                        );
                        if let Err(e) = ArchiveNodeNatsClient::publish(block_packet).await {
                            tracing::error!(
                                "Failed to publish block:{:?} due to error: {:?}",
                                block_height,
                                e
                            );
                            return Err(anyhow::anyhow!(
                                "Failed to publish block: {:?}",
                                block_height
                            ));
                        }

                        tracing::info!("Block:{:?} published successfully", block_height);

                        Ok(())
                    }
                })
                .buffered(10)
                .try_collect::<()>() // Ensure all futures are resolved
                .await?;

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
