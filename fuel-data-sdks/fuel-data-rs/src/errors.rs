
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FuelDataEdgeError {
    #[error("gRPC connection error: {0}")]
    GrpcConnectionError(#[from] GrpcConnectionError),
    #[error("Stream filter error: {0}")]
    StreamFilterError(#[from] StreamFilterError),
}

#[derive(Debug, Error)]
pub enum GrpcConnectionError {
    #[error("Network timeout occurred")]
    Timeout,
    #[error("Connection was refused")]
    ConnectionRefused,
    #[error("Invalid gRPC URI: {0}")]
    InvalidGrpcUri(#[from] tonic::transport::Error), // Renamed for clarity and standard naming
}

#[derive(Debug, Error)]
pub enum StreamFilterError {
    #[error("Invalid filter: {0}")]
    InvalidFilter(String),
}
