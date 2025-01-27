use fuel_data_protos::fuel_data_edge::filters::BlocksFilterProto;
use fuel_data_protos::fuel_data_grpc_edge::streams::blocks_stream_client::BlocksStreamClient;
use fuel_data_protos::fuel_data_grpc_edge::streams::BlocksStreamRequest;
use fuel_data_types::{Address, Block};
use tokio_stream::{Stream, StreamExt};

use crate::edge::FuelDataEdge;
use crate::errors::{FuelDataError, GrpcConnectionError, StreamFilterError};

#[derive(Debug, Clone, Default)]
pub struct BlocksFilter {
    from: Option<u32>,
    to: Option<u32>,
    producer: Option<Address>,
    take: Option<u16>,
}

impl Into<BlocksFilterProto> for BlocksFilter {
    fn into(self) -> BlocksFilterProto {
        BlocksFilterProto {
            from: self.from,
            to: self.to,
            producer: self.producer.map(|a| a.to_string()),
            take: self.take.map(Into::into),
            // TODO: Support chunking in newer versions
            ..Default::default()
        }
    }
}

impl BlocksFilter {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from(mut self, value: u32) -> Self {
        self.from = Some(value);
        self
    }
    pub fn to(mut self, value: u32) -> Self {
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

    pub async fn stream(
        self,
        edge: &FuelDataEdge,
    ) -> Result<impl Stream<Item = Result<Block, FuelDataError>>, FuelDataError> {
        if self.from > self.to {
            return Err(StreamFilterError::InvalidFilter(
                "Invalid filter: 'from' must be less than 'to'".to_string(),
            ))?;
        }

        let filter_proto: BlocksFilterProto = self.into();
        let grpc_request = BlocksStreamRequest {
            filter: Some(filter_proto),
        };

        let mut connection = BlocksStreamClient::connect(edge.grpc_endpoint.clone())
            .await
            .map_err(GrpcConnectionError::from)?;

        let tonic_stream = connection
            .get(grpc_request)
            .await
            .map_err(FuelDataError::from)?
            .into_inner();

        let stream = tonic_stream.map(|item| {
            item.map(|stream_item_proto| {
                let stream_item: Block = stream_item_proto.into();
                stream_item
            })
            .map_err(FuelDataError::from)
        });

        Ok(stream)
    }
}
