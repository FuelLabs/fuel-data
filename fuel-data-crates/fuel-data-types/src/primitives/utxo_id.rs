use super::Bytes32;

use fuel_node::types::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UtxoId {
    pub tx_id: Bytes32,
    pub output_index: u16,
}

impl From<FuelNodeUtxoId> for UtxoId {
    fn from(value: FuelNodeUtxoId) -> Self {
        Self::from(&value)
    }
}

impl From<&FuelNodeUtxoId> for UtxoId {
    fn from(value: &FuelNodeUtxoId) -> Self {
        Self {
            tx_id: value.tx_id().into(),
            output_index: value.output_index(),
        }
    }
}
