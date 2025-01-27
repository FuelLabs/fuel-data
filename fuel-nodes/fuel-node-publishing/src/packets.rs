use std::sync::Arc;

use fuel_data_subjects::{BlocksSubject, Subject};
use fuel_data_types::*;
use fuel_node::types::*;

pub struct Packet<Proto: prost::Message> {
    pub nats_subject: String,
    pub payload: Proto,
}

impl<Proto: prost::Message> Packet<Proto> {
    pub fn new(subject: impl Subject<DataTypeProto = Proto>, payload: Proto) -> Self {
        Self {
            nats_subject: subject.to_nats_subject(),
            payload,
        }
    }
}

pub struct BuildPacketCommonArgs {
    pub block_producer: Arc<Address>,
    pub block_height: u32,
    pub consensus: Arc<ConsensusProto>,
}

pub fn build_block_packet(
    block: &FuelNodeBlock,
    transaction_ids: Vec<Bytes32Proto>,
    build_args: BuildPacketCommonArgs,
) -> Packet<BlockProto> {
    let block_height = build_args.block_height;
    let block_producer = (*build_args.block_producer).clone();
    let consensus = (*build_args.consensus).clone();

    let block = BlockProto::new(block, consensus, transaction_ids);
    let subject = BlocksSubject {
        producer: block_producer,
        block_height,
    };

    Packet::new(subject, block)
}
