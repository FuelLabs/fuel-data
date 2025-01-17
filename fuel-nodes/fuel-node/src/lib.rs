#[cfg(feature = "all")]
pub mod fuel_node;

#[cfg(feature = "all")]
pub mod fuel_node_like;

#[cfg(feature = "all")]
pub use fuel_node::FuelNode;

#[cfg(feature = "all")]
pub use fuel_node_like::FuelNodeLike;

#[cfg(feature = "types")]
pub mod fuel_node_types;

#[cfg(feature = "types")]
pub mod types {
    pub use super::fuel_node_types::*;
}
