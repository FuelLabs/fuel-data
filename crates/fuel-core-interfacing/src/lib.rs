#[cfg(feature = "all")]
pub mod fuel_core;

#[cfg(feature = "all")]
pub mod fuel_core_like;

#[cfg(feature = "all")]
pub use fuel_core::FuelCore;

#[cfg(feature = "all")]
pub use fuel_core_like::FuelCoreLike;

#[cfg(feature = any("all", "types"))]
pub mod fuel_core_types;

#[cfg(feature = any("all", "types"))]
pub mod types {
    pub use fuel_core_types::*;
}
