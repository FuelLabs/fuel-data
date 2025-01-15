use std::{sync::Arc, time::Duration};

use fuel_core::{
    combined_database::CombinedDatabase,
    state::{
        generic_database::GenericDatabase, iterable_key_value_view::IterableKeyValueViewWrapper,
    },
};
use fuel_core_bin::FuelService;
use tokio::{sync::broadcast::Receiver, time::sleep};

use crate::fuel_node_like::FuelNodeLike;
use crate::fuel_node_types::*;

pub type OffchainDatabase =
    GenericDatabase<IterableKeyValueViewWrapper<fuel_core::fuel_core_graphql_api::storage::Column>>;

#[derive(Clone)]
pub struct FuelNode {
    pub fuel_service: Arc<FuelService>,
    chain_id: FuelNodeChainId,
    base_asset_id: FuelNodeAssetId,
    database: CombinedDatabase,
}

impl From<FuelService> for FuelNode {
    fn from(fuel_service: FuelService) -> Self {
        let chain_config = fuel_service.shared.config.snapshot_reader.chain_config();
        let chain_id = chain_config.consensus_parameters.chain_id();
        let base_asset_id = *chain_config.consensus_parameters.base_asset_id();

        let database = fuel_service.shared.database.clone();

        Self {
            fuel_service: Arc::new(fuel_service),
            chain_id,
            base_asset_id,
            database,
        }
    }
}

impl FuelNode {
    pub async fn new(command: fuel_core_bin::cli::run::Command) -> anyhow::Result<Arc<Self>> {
        let fuel_service = fuel_core_bin::cli::run::get_service(command).await?;

        let fuel_core: Self = fuel_service.into();

        Ok(fuel_core.arc())
    }
    pub fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[async_trait::async_trait]
impl FuelNodeLike for FuelNode {
    async fn start(&self) -> anyhow::Result<()> {
        fuel_core_bin::cli::init_logging();

        self.fuel_service.start_and_await().await?;

        Ok(())
    }
    fn is_started(&self) -> bool {
        self.fuel_service.state().started()
    }
    async fn await_synced_at_least_once(&self, historical: bool) -> anyhow::Result<()> {
        if !historical {
            self.fuel_service.await_relayer_synced().await?;
        }
        Ok(())
    }

    async fn stop(&self) {
        if matches!(
            self.fuel_service.state(),
            fuel_core_services::State::Stopped
                | fuel_core_services::State::Stopping
                | fuel_core_services::State::StoppedWithError(_)
                | fuel_core_services::State::NotStarted
        ) {
            return;
        }

        tracing::info!("Stopping fuel core ...");
        match self
            .fuel_service
            .send_stop_signal_and_await_shutdown()
            .await
        {
            Ok(state) => {
                tracing::info!("Stopped fuel core. Status = {:?}", state)
            }
            Err(e) => tracing::error!("Stopping fuel core failed: {:?}", e),
        }
    }

    async fn await_offchain_db_sync(&self, block_id: &FuelNodeBlockId) -> anyhow::Result<()> {
        loop {
            if self
                .offchain_database()?
                .get_block_height(block_id)?
                .is_some()
            {
                break;
            };

            sleep(Duration::from_millis(500)).await;
        }

        Ok(())
    }

    fn base_asset_id(&self) -> &FuelNodeAssetId {
        &self.base_asset_id
    }
    fn chain_id(&self) -> &FuelNodeChainId {
        &self.chain_id
    }

    fn database(&self) -> &CombinedDatabase {
        &self.database
    }

    fn blocks_subscription(&self) -> Receiver<fuel_core_importer::ImporterResult> {
        self.fuel_service
            .shared
            .block_importer
            .block_importer
            .subscribe()
    }

    fn get_receipts(
        &self,
        tx_id: &FuelNodeBytes32,
    ) -> anyhow::Result<Option<Vec<FuelNodeReceipt>>> {
        let receipts = self
            .offchain_database()?
            .get_tx_status(tx_id)?
            .map(|status| match &status {
                FuelNodeTransactionStatus::Success { receipts, .. }
                | FuelNodeTransactionStatus::Failed { receipts, .. } => Some(receipts.clone()),
                _ => None,
            })
            .unwrap_or_default();

        Ok(receipts)
    }
}
