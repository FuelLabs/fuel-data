use std::sync::Arc;

use fuel_core::{
    combined_database::CombinedDatabase,
    database::{
        database_description::{on_chain::OnChain, DatabaseHeight},
        Database,
    },
    fuel_core_graphql_api::ports::DatabaseBlocks,
    state::{
        generic_database::GenericDatabase, iterable_key_value_view::IterableKeyValueViewWrapper,
    },
};
use fuel_core_importer::ports::ImporterDatabase;
use fuel_core_storage::transactional::AtomicView;
use fuel_core_types::{
    blockchain::consensus::{Consensus, Sealed},
    fuel_types::BlockHeight,
};
use tokio::sync::broadcast::Receiver;

use crate::fuel_node_types::*;

pub type OffchainDatabase =
    GenericDatabase<IterableKeyValueViewWrapper<fuel_core::fuel_core_graphql_api::storage::Column>>;

/// Interface for `fuel-core` related logic.
/// This was introduced to simplify mocking and testing the `fuel-streams-publisher` crate.
#[async_trait::async_trait]
pub trait FuelNodeLike: Sync + Send {
    async fn start(&self) -> anyhow::Result<()>;
    fn is_started(&self) -> bool;
    async fn await_synced_at_least_once(&self) -> anyhow::Result<()>;
    async fn stop(&self);

    fn base_asset_id(&self) -> &FuelNodeAssetId;
    fn chain_id(&self) -> &FuelNodeChainId;

    fn database(&self) -> &CombinedDatabase;
    fn onchain_database(&self) -> &Database<OnChain> {
        self.database().on_chain()
    }
    fn offchain_database(&self) -> anyhow::Result<Arc<OffchainDatabase>> {
        Ok(Arc::new(self.database().off_chain().latest_view()?))
    }

    async fn await_offchain_db_sync(&self, block_id: &FuelNodeBlockId) -> anyhow::Result<()>;

    fn blocks_subscription(&self) -> Receiver<fuel_core_importer::ImporterResult>;

    fn get_latest_block_height(&self) -> anyhow::Result<u64> {
        Ok(self
            .onchain_database()
            .latest_block_height()?
            .map(|h| h.as_u64())
            .unwrap_or_default())
    }

    fn get_receipts(&self, tx_id: &FuelNodeBytes32)
        -> anyhow::Result<Option<Vec<FuelNodeReceipt>>>;

    #[cfg(not(feature = "test-helpers"))]
    fn get_consensus(&self, block_height: &BlockHeight) -> anyhow::Result<Consensus> {
        Ok(self
            .onchain_database()
            .latest_view()?
            .consensus(block_height)?)
    }

    #[cfg(feature = "test-helpers")]
    fn get_consensus(&self, block_height: &BlockHeight) -> anyhow::Result<Consensus> {
        Ok(self
            .onchain_database()
            .latest_view()?
            .consensus(block_height)
            .unwrap_or_default())
    }

    #[cfg(not(feature = "test-helpers"))]
    fn get_block_and_producer(
        &self,
        sealed_block: Sealed<FuelNodeBlock>,
    ) -> (FuelNodeBlock, FuelNodeAddress) {
        let block = sealed_block.entity.clone();
        let block_producer = sealed_block
            .consensus
            .block_producer(&block.id())
            .expect("Failed to get Block Producer");

        (block, block_producer.into())
    }

    #[cfg(feature = "test-helpers")]
    fn get_block_and_producer(
        &self,
        sealed_block: Sealed<FuelNodeBlock>,
    ) -> (FuelNodeBlock, Address) {
        let block = sealed_block.entity.clone();
        let block_producer = sealed_block
            .consensus
            .block_producer(&block.id())
            .unwrap_or_default();

        (block, block_producer.into())
    }

    fn get_sealed_block_by_height(&self, height: u32) -> Sealed<FuelNodeBlock> {
        self.onchain_database()
            .latest_view()
            .expect("failed to get latest db view")
            .get_sealed_block_by_height(&height.into())
            .expect("Failed to get latest block height")
            .expect("NATS Publisher: no block at height {height}")
    }
}
