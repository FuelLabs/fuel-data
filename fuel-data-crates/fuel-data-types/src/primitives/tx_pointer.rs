use fuel_node_types::*;

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct TxPointer {
    block_height: FuelNodeBlockHeight,
    tx_index: u16,
}

impl From<FuelNodeTxPointer> for TxPointer {
    fn from(value: FuelNodeTxPointer) -> Self {
        Self {
            block_height: value.block_height(),
            tx_index: value.tx_index(),
        }
    }
}
