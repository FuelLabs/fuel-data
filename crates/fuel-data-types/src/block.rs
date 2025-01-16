use serde::{Deserialize, Serialize};

use super::primitives::*;
use fuel_node::types::*;

// Block type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub consensus: Consensus,
    pub header: BlockHeader,
    pub height: u32,
    pub id: BlockId,
    pub transaction_ids: Vec<Bytes32>,
    pub version: BlockVersion,
}

impl Block {
    pub fn new(block: &FuelNodeBlock, consensus: Consensus, transaction_ids: Vec<Bytes32>) -> Self {
        let header: BlockHeader = block.header().into();
        let height = header.height;

        let version = match block {
            FuelNodeBlock::V1(_) => BlockVersion::V1,
        };

        Self {
            consensus,
            header: header.to_owned(),
            height,
            id: header.id,
            transaction_ids,
            version,
        }
    }
}

// Consensus enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Consensus {
    Genesis(Genesis),
    PoAConsensus(PoAConsensus),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genesis {
    pub chain_config_hash: Bytes32,
    pub coins_root: Bytes32,
    pub contracts_root: Bytes32,
    pub messages_root: Bytes32,
    pub transactions_root: Bytes32,
}

impl From<FuelNodeGenesis> for Genesis {
    fn from(genesis: FuelNodeGenesis) -> Self {
        Self {
            chain_config_hash: genesis.chain_config_hash.into(),
            coins_root: genesis.coins_root.into(),
            contracts_root: genesis.contracts_root.into(),
            messages_root: genesis.messages_root.into(),
            transactions_root: genesis.transactions_root.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoAConsensus {
    pub signature: Signature,
}

impl PoAConsensus {
    pub fn new(signature: Signature) -> Self {
        Self { signature }
    }
}

impl From<FuelNodePoAConsensus> for PoAConsensus {
    fn from(poa: FuelNodePoAConsensus) -> Self {
        Self {
            signature: Signature(poa.signature.into()),
        }
    }
}

impl Default for Consensus {
    fn default() -> Self {
        Consensus::Genesis(Genesis::default())
    }
}

impl From<FuelNodeConsensus> for Consensus {
    fn from(consensus: FuelNodeConsensus) -> Self {
        match consensus {
            FuelNodeConsensus::Genesis(genesis) => Consensus::Genesis(genesis.into()),
            FuelNodeConsensus::PoA(poa) => Consensus::PoAConsensus(poa.into()),
            _ => panic!("Unknown consensus type: {:?}", consensus),
        }
    }
}

// BlockVersion enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockVersion {
    V1,
}

// Header type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeader {
    pub application_hash: Bytes32,
    pub consensus_parameters_version: u32,
    pub da_height: u64,
    pub event_inbox_root: Bytes32,
    pub id: BlockId,
    pub height: u32,
    pub message_outbox_root: Bytes32,
    pub message_receipt_count: u32,
    pub prev_root: Bytes32,
    pub state_transition_bytecode_version: u32,
    pub time: FuelNodeTai64Timestamp,
    pub transactions_count: u16,
    pub transactions_root: Bytes32,
    pub version: BlockHeaderVersion,
}

impl From<&FuelNodeBlockHeader> for BlockHeader {
    fn from(header: &FuelNodeBlockHeader) -> Self {
        let version = match header {
            FuelNodeBlockHeader::V1(_) => BlockHeaderVersion::V1,
        };

        Self {
            application_hash: (*header.application_hash()).into(),
            consensus_parameters_version: header.consensus_parameters_version,
            da_height: header.da_height.into(),
            event_inbox_root: header.event_inbox_root.into(),
            id: header.id().into(),
            height: (*header.height()).into(),
            message_outbox_root: header.message_outbox_root.into(),
            message_receipt_count: header.message_receipt_count,
            prev_root: (*header.prev_root()).into(),
            state_transition_bytecode_version: header.state_transition_bytecode_version,
            time: header.time().into(),
            transactions_count: header.transactions_count,
            transactions_root: header.transactions_root.into(),
            version,
        }
    }
}

// BlockHeaderVersion enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockHeaderVersion {
    V1,
}
