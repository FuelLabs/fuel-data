use fuel_data_protos::fuel_data_grpc_edge::streams::blocks_stream_client::BlocksStreamClient;
use fuel_data_protos::fuel_data_grpc_edge::streams::BlocksStreamRequest;
use fuel_data_types::{Address, Block};
use tokio_stream::{Stream, StreamExt};

use crate::edge::FuelDataEdge;
use crate::errors::{FuelDataError, GrpcConnectionError};

#[derive(Debug, Clone, Default)]
pub struct BlocksStream {
    from: Option<u64>,
    to: Option<u64>,
    producer: Option<Address>,
    take: Option<u16>,
    chunk: Option<u16>,
}

impl BlocksStream {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from(mut self, value: u64) -> Self {
        self.from = Some(value);
        self
    }
    pub fn to(mut self, value: u64) -> Self {
        self.to = Some(value);
        self
    }
    pub fn producer(mut self, value: Address) -> Self {
        self.producer = Some(value);
        self
    }
    pub fn take(mut self, count: u16) -> Self {
        self.take = Some(count);
        self
    }
    pub fn chunk(mut self, count: u16) -> Self {
        self.chunk = Some(count);
        self
    }

    pub async fn stream(
        self,
        edge: &FuelDataEdge,
    ) -> Result<impl Stream<Item = Result<Block, FuelDataError>>, FuelDataError> {
        // TODO: Validate the filter
        // TODO: Extract query from filter

        let grpc_request = BlocksStreamRequest::default();
        let mut connection = BlocksStreamClient::connect(edge.grpc_endpoint.clone())
            .await
            .map_err(GrpcConnectionError::from)?;

        let tonic_stream = connection
            .get(grpc_request)
            .await
            .map_err(FuelDataError::from)?
            .into_inner();

        // Map the gRPC stream to a stream of `Result<Block, FuelDataError>`
        let stream = tonic_stream.map(|item| {
            item.map(|stream_item_proto| {
                let stream_item: Block = stream_item_proto.into();
                stream_item
            })
            .map_err(FuelDataError::from) // Convert `Status` to `FuelDataError`
        });

        Ok(stream)
    }
}
