use std::sync::Arc;

use fuel_data_types::*;
use fuel_node::types::*;

use crate::subjects::{BlocksSubject, Subject};

pub struct Packet<T: prost::Message> {
    pub subject: Subject,
    pub payload: T,
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

    Packet {
        subject: Subject::Blocks(BlocksSubject {
            producer: block_producer,
            block_height,
        }),
        payload: block,
    }
}
