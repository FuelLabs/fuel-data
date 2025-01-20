use fuel_node_types::*;

include!("../../generated/rust/fuel_data_types.block.rs");

impl BlockProto {
    pub fn new(
        block: &FuelNodeBlock,
        consensus: ConsensusProto,
        transaction_ids: Vec<Bytes32Proto>,
    ) -> Self {
        let header: BlockHeaderProto = block.header().into();
        let height = header.height;

        let version = match block {
            FuelNodeBlock::V1(_) => BlockVersionProto::BlockVersionV1,
        };

        Self {
            consensus: Some(consensus),
            header: Some(header.to_owned()),
            height,
            id: header.id,
            transaction_ids,
            version: version.into(),
        }
    }
}

impl From<FuelNodeGenesis> for GenesisProto {
    fn from(genesis: FuelNodeGenesis) -> Self {
        Self {
            chain_config_hash: Some(Bytes32Proto {
                value: genesis.chain_config_hash.to_vec(),
            }),
            coins_root: Some(Bytes32Proto {
                value: genesis.coins_root.to_vec(),
            }),
            contracts_root: Some(Bytes32Proto {
                value: genesis.contracts_root.to_vec(),
            }),
            messages_root: Some(Bytes32Proto {
                value: genesis.messages_root.to_vec(),
            }),
            transactions_root: Some(Bytes32Proto {
                value: genesis.transactions_root.to_vec(),
            }),
        }
    }
}

impl From<FuelNodePoAConsensus> for PoAConsensusProto {
    fn from(poa: FuelNodePoAConsensus) -> Self {
        Self {
            signature: Some(SignatureProto {
                value: poa.signature.to_vec(),
            }),
        }
    }
}

impl From<FuelNodeConsensus> for ConsensusProto {
    fn from(consensus: FuelNodeConsensus) -> Self {
        match consensus {
            FuelNodeConsensus::Genesis(genesis) => ConsensusProto {
                consensus_type: Some(consensus_proto::ConsensusType::Genesis(GenesisProto::from(
                    genesis,
                ))),
            },
            FuelNodeConsensus::PoA(poa) => ConsensusProto {
                consensus_type: Some(consensus_proto::ConsensusType::PoaConsensus(
                    PoAConsensusProto::from(poa),
                )),
            },
            _ => panic!("Unknown consensus type"),
        }
    }
}

impl From<&FuelNodeBlockHeader> for BlockHeaderProto {
    fn from(header: &FuelNodeBlockHeader) -> Self {
        Self {
            application_hash: Some(Bytes32Proto {
                value: header.application_hash().to_vec(),
            }),
            consensus_parameters_version: header.consensus_parameters_version,
            da_height: *header.da_height,
            event_inbox_root: Some(Bytes32Proto {
                value: header.event_inbox_root.to_vec(),
            }),
            id: Some(header.id().into()),
            height: **header.height(),
            message_outbox_root: Some(Bytes32Proto {
                value: header.message_outbox_root.to_vec(),
            }),
            message_receipt_count: header.message_receipt_count,
            prev_root: Some(Bytes32Proto {
                value: header.prev_root().to_vec(),
            }),
            state_transition_bytecode_version: header.state_transition_bytecode_version,
            time: Some(Tai64TimestampProto {
                value: header.time().0,
            }),
            transactions_count: header.transactions_count as u32,
            transactions_root: Some(Bytes32Proto {
                value: header.transactions_root.to_vec(),
            }),
            version: BlockHeaderVersionProto::BlockHeaderVersionV1 as i32,
        }
    }
}

impl From<[u8; 32]> for Bytes32Proto {
    fn from(value: [u8; 32]) -> Self {
        Self {
            value: value.to_vec(),
        }
    }
}

impl From<FuelNodeBlockId> for BlockIdProto {
    fn from(id: FuelNodeBlockId) -> Self {
        let id: [u8; 32] = id.into();
        Self { value: id.to_vec() }
    }
}

impl From<FuelNodeSignature> for SignatureProto {
    fn from(signature: FuelNodeSignature) -> Self {
        Self {
            value: signature.to_vec(),
        }
    }
}

impl From<u64> for Tai64TimestampProto {
    fn from(timestamp: u64) -> Self {
        Self { value: timestamp }
    }
}
