mod errors;
mod networks;
mod streams;

use std::convert::TryInto;

use tokio_stream::StreamExt;

use fuel_data_protos::fuel_data_grpc_edge::streams::blocks_stream_client::BlocksStreamClient;
use fuel_data_protos::fuel_data_grpc_edge::streams::BlocksStreamRequest;

use errors::{FuelDataEdgeError, GrpcConnectionError, StreamFilterError};

pub struct FuelDataEdge {}

impl FuelDataEdge {
    pub fn connect<E: TryInto<tonic::transport::Endpoint>>(
        grpc_endpoint: E,
    ) -> Result<Self, FuelDataEdgeError> {
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
