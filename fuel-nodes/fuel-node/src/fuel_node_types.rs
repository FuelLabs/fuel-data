use fuel_core::state::{
    generic_database::GenericDatabase, iterable_key_value_view::IterableKeyValueViewWrapper,
};
pub use fuel_core_client::client::{
    schema::Tai64Timestamp as FuelNodeTai64Timestamp,
    types::TransactionStatus as FuelNodeClientTransactionStatus,
};
pub use fuel_core_importer::ImporterResult as FuelNodeImporterResult;
pub use fuel_core_types::tai64::Tai64N as FuelNodeTai64N;
pub use fuel_core_types::{
    blockchain::{
        block::Block as FuelNodeBlock,
        consensus::{
            poa::PoAConsensus as FuelNodePoAConsensus, Consensus as FuelNodeConsensus,
            Genesis as FuelNodeGenesis,
        },
        header::BlockHeader as FuelNodeBlockHeader,
        primitives::BlockId as FuelNodeBlockId,
        SealedBlock as FuelNodeSealedBlock,
    },
    fuel_asm::Word as FuelNodeWord,
    fuel_crypto::Signature as FuelNodeSignature,
    fuel_tx::{
        field::{Inputs as FuelNodeInputs, Outputs as FuelNodeOutputs},
        input::contract::Contract as FuelNodeInputContract,
        output::contract::Contract as FuelNodeOutputContract,
        policies::Policies as FuelNodePolicies,
        Address as FuelNodeAddress, AssetId as FuelNodeAssetId, BlobId as FuelNodeBlobId,
        Bytes32 as FuelNodeBytes32, Contract as FuelNodeContract, ContractId as FuelNodeContractId,
        Input as FuelNodeInput, MessageId as FuelNodeMessageId, Output as FuelNodeOutput,
        PanicInstruction as FuelNodePanicInstruction, Receipt as FuelNodeReceipt,
        ScriptExecutionResult as FuelNodeScriptExecutionResult, StorageSlot as FuelNodeStorageSlot,
        Transaction as FuelNodeTransaction, TxId as FuelNodeTxId, TxPointer as FuelNodeTxPointer,
        UniqueIdentifier as FuelNodeUniqueIdentifier, UpgradePurpose as FuelNodeUpgradePurpose,
        UtxoId as FuelNodeUtxoId,
    },
    fuel_types::{BlockHeight as FuelNodeBlockHeight, ChainId as FuelNodeChainId},
    services::{
        block_importer::{
            ImportResult as FuelNodeImportResult, SharedImportResult as FuelNodeSharedImportResult,
        },
        txpool::TransactionStatus as FuelNodeTransactionStatus,
    },
    tai64::Tai64 as FuelNodeTai64,
};

pub type FuelNodeOffchainDatabase =
    GenericDatabase<IterableKeyValueViewWrapper<fuel_core::fuel_core_graphql_api::storage::Column>>;
