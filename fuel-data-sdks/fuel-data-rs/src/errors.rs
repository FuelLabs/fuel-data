use thiserror::Error;

#[derive(Debug, Error)]
pub enum FuelDataError {
    #[error("gRPC server error: {0}")]
    ServerError(#[from] tonic::Status),
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
    InvalidGrpcUri(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
}

#[derive(Debug, Error)]
pub enum StreamFilterError {
    #[error("Invalid filter: {0}")]
    InvalidFilter(String),
}
