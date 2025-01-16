use std::sync::Arc;

use crate::nats_client::RelayNodeNatsClient;
use crate::packets::build_block_packet;
use crate::packets::BuildPacketCommonArgs;

use fuel_core::database::database_description::DatabaseHeight;
use fuel_data_types::*;
use fuel_node::types::*;

use fuel_node::FuelNodeLike;

pub struct RelayNode {
    pub fuel_core: Arc<dyn FuelNodeLike>,
    pub nats_client: Arc<RelayNodeNatsClient>,
}

impl RelayNode {
    pub async fn new(fuel_core: Arc<dyn FuelNodeLike>) -> anyhow::Result<Self> {
        let nats_client = RelayNodeNatsClient::connect().await?;

        fuel_core.start().await?;

        Ok(Self {
            fuel_core,
            nats_client: Arc::new(nats_client),
        })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let mut blocks_subscription = self.fuel_core.blocks_subscription();

        let fuel_core = &self.fuel_core;
        let _base_asset_id = Arc::new(*fuel_core.base_asset_id());

        while let Ok(importer_result) = blocks_subscription.recv().await {
            let sealed_block = importer_result.sealed_block.clone();
            let (block, block_producer) = fuel_core.get_block_and_producer(sealed_block);
            let block_producer = Arc::new(block_producer.into());
            let block_height = block.header().consensus().height;
            let transactions = block.transactions();
            let chain_id = Arc::new(*fuel_core.chain_id());
            let transaction_ids = transactions
                .iter()
                .map(|tx| tx.id(&chain_id).into())
                .map(|tx_id: [u8; 32]| tx_id.into())
                .collect::<Vec<Bytes32Proto>>();
            let consensus: ConsensusProto = self.fuel_core.get_consensus(&block_height)?.into();
            let block_height = block_height.as_u64() as u32;

            let build_args = BuildPacketCommonArgs {
                block_producer,
                block_height,
                consensus: Arc::new(consensus),
            };
            let block_packet = build_block_packet(&block, transaction_ids, build_args);

            tracing::info!("Publishing block:{:?}...", block_height);
            self.nats_client.publish_live(block_packet).await?;
            tracing::info!("Block:{:?} published successfully", block_height);
        }

        Ok(())
    }
}
