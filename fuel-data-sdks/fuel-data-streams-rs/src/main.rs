use tokio_stream::StreamExt;

use fuel_data_protos::fuel_data_grpc_edge::streams::blocks_stream_client::BlocksStreamClient;
use fuel_data_protos::fuel_data_grpc_edge::streams::BlocksStreamRequest;

mod errors {
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
}

use errors::{FuelDataEdgeError, GrpcConnectionError, StreamFilterError};
use std::convert::TryInto;
use tonic::transport::Endpoint;

pub enum FuelDataGrpcEdgeNetwork {
    Local,
    Remote,
}

impl TryInto<Endpoint> for FuelDataGrpcEdgeNetwork {
    type Error = GrpcConnectionError;

    fn try_into(self) -> Result<Endpoint, Self::Error> {
        match self {
            FuelDataGrpcEdgeNetwork::Local => "http://[::1]:50051"
                .parse()
                .map_err(GrpcConnectionError::from),
            FuelDataGrpcEdgeNetwork::Remote => "http://fuel-data-grpc-edge.fuel.network:50051"
                .parse()
                .map_err(GrpcConnectionError::from),
        }
    }
}

pub struct FuelDataEdge {}

impl FuelDataEdge {
    pub fn connect<E: TryInto<tonic::transport::Endpoint>>(
        grpc_endpoint: E,
    ) -> Result<Self, FuelDataEdgeError> {
        Ok(Self {})
    }
    // pub fn build() -> Result<>
}

trait StreamFilter {
    fn take(&mut self, count: u16) -> Self;
    fn chunk(&mut self, count: u16) -> Self;
    fn build(&self) -> Result<Self, StreamFilterError>
    where
        Self: Sized;
}

struct BlocksStream {}

impl BlocksStream {
    pub fn new() -> Self {
        Self {}
    }
    pub fn from() -> Self {
        Self {}
    }
    pub fn to() -> Self {
        Self {}
    }
    pub fn producer() -> Self {
        Self {}
    }
}

impl StreamFilter for BlocksStream {
    fn take(&mut self, count: u16) -> Self {
        Self {}
    }

    fn chunk(&mut self, count: u16) -> Self {
        Self {}
    }

    fn build(&self) -> Result<Self, StreamFilterError> {
        Ok(Self {})
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BlocksStreamClient::connect("http://[::1]:50051").await?;

    let mut stream = client
        .get(BlocksStreamRequest::default())
        .await
        .unwrap()
        .into_inner();

    while let Some(response) = stream.next().await {
        println!("\treceived: {}", serde_json::to_string(&response.ok())?);
    }

    Ok(())
}
