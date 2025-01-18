use serde::{Deserialize, Serialize};

use fuel_data_protos::fuel_data_types::*;

use fuel_node::types::*;

use crate::primitives::*;

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

impl From<BlockProto> for Block {
    fn from(proto: BlockProto) -> Self {
        let consensus = proto
            .consensus
            .map(Consensus::from)
            .expect("Consensus is required in BlockProto");

        let header = proto
            .header
            .map(BlockHeader::from)
            .expect("Header is required in BlockProto");

        let transaction_ids = proto
            .transaction_ids
            .into_iter()
            .map(Bytes32::from)
            .collect();

        let version = match proto.version {
            0 => BlockVersion::V1,
            _ => panic!("Unknown BlockVersionProto"),
        };

        Self {
            consensus,
            header,
            height: proto.height,
            id: proto
                .id
                .map(BlockId::from)
                .expect("BlockId is required in BlockProto"),
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

impl From<ConsensusProto> for Consensus {
    fn from(proto: ConsensusProto) -> Self {
        match proto.consensus_type {
            Some(consensus_proto::ConsensusType::Genesis(genesis_proto)) => {
                Consensus::Genesis(Genesis::from(genesis_proto))
            }
            Some(consensus_proto::ConsensusType::PoaConsensus(poa_proto)) => {
                Consensus::PoAConsensus(PoAConsensus::from(poa_proto))
            }
            None => panic!("Unknown consensus type in ConsensusProto"),
        }
    }
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

impl From<GenesisProto> for Genesis {
    fn from(proto: GenesisProto) -> Self {
        Self {
            chain_config_hash: proto
                .chain_config_hash
                .map(Bytes32::from)
                .expect("Chain config hash is required in GenesisProto"),
            coins_root: proto
                .coins_root
                .map(Bytes32::from)
                .expect("Coins root is required in GenesisProto"),
            contracts_root: proto
                .contracts_root
                .map(Bytes32::from)
                .expect("Contracts root is required in GenesisProto"),
            messages_root: proto
                .messages_root
                .map(Bytes32::from)
                .expect("Messages root is required in GenesisProto"),
            transactions_root: proto
                .transactions_root
                .map(Bytes32::from)
                .expect("Transactions root is required in GenesisProto"),
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

impl From<PoAConsensusProto> for PoAConsensus {
    fn from(proto: PoAConsensusProto) -> Self {
        Self {
            signature: proto
                .signature
                .map(Signature::from)
                .expect("Signature is required in PoAConsensusProto"),
        }
    }
}

impl Default for Consensus {
    fn default() -> Self {
        Consensus::Genesis(Genesis::default())
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

impl From<BlockHeaderProto> for BlockHeader {
    fn from(proto: BlockHeaderProto) -> Self {
        let version = match proto.version {
            v if v == BlockHeaderVersionProto::BlockHeaderVersionV1 as i32 => {
                BlockHeaderVersion::V1
            }
            _ => panic!("Unknown BlockHeaderVersionProto"),
        };

        Self {
            application_hash: proto
                .application_hash
                .map(Bytes32::from)
                .expect("Application hash is required in BlockHeaderProto"),
            consensus_parameters_version: proto.consensus_parameters_version,
            da_height: proto.da_height,
            event_inbox_root: proto
                .event_inbox_root
                .map(Bytes32::from)
                .expect("Event inbox root is required in BlockHeaderProto"),
            id: proto.id.map(BlockId::from).expect("BlockId is required"),
            height: proto.height,
            message_outbox_root: proto
                .message_outbox_root
                .map(Bytes32::from)
                .expect("Message outbox root is required"),
            message_receipt_count: proto.message_receipt_count,
            prev_root: proto
                .prev_root
                .map(Bytes32::from)
                .expect("Previous root is required"),
            state_transition_bytecode_version: proto.state_transition_bytecode_version,
            time: proto
                .time
                .map(|time_proto| FuelNodeTai64Timestamp(tai64::Tai64(time_proto.value)))
                .expect("Timestamp is required"),
            transactions_count: proto.transactions_count as u16,
            transactions_root: proto
                .transactions_root
                .map(Bytes32::from)
                .expect("Transactions root is required"),
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

// Shared Types
impl From<Bytes32Proto> for Bytes32 {
    fn from(proto: Bytes32Proto) -> Self {
        let mut value = [0u8; 32];
        value.copy_from_slice(&proto.value);
        value.into()
    }
}

impl From<BlockIdProto> for BlockId {
    fn from(proto: BlockIdProto) -> Self {
        let mut value = [0u8; 32];
        value.copy_from_slice(&proto.value);
        value.into()
    }
}

impl From<SignatureProto> for Signature {
    fn from(proto: SignatureProto) -> Self {
        let mut value = [0u8; 64];
        value.copy_from_slice(&proto.value);
        value.into()
    }
}
