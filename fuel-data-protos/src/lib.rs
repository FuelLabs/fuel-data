#[cfg(feature = "fuel-data-types")]
pub mod fuel_data_types;

#[cfg(feature = "fuel-data-edge")]
pub mod fuel_data_edge {
    pub mod filters {
        include!("../generated/rust/fuel_data_edge.filters.rs");
    }
}

#[cfg(feature = "fuel-data-grpc-edge")]
pub mod fuel_data_grpc_edge {
    pub mod streams {
        include!("../generated/rust/fuel_data_grpc_edge.streams.rs");
    }
}
