use std::pin::Pin;

use fuel_data_subjects::BlocksSubjectFilter;
use tonic::{Request, Response, Status};

use fuel_data_protos::fuel_data_grpc_edge::streams::BlocksStreamRequest;
use fuel_data_protos::fuel_data_types::BlockProto;
use fuel_data_protos::{
    fuel_data_edge::filters::BlocksFilterProto,
    fuel_data_grpc_edge::streams::blocks_stream_server::BlocksStream,
};

use tokio_stream::{Stream, StreamExt};

use prost::Message;

use fuel_data_edge::EdgeNatsClient;

#[derive(Debug, Default)]
pub struct BlocksStreamer;

#[tonic::async_trait]
impl BlocksStream for BlocksStreamer {
    type GetStream = Pin<Box<dyn Stream<Item = Result<BlockProto, Status>> + Send>>;

    async fn get(
        &self,
        request: Request<BlocksStreamRequest>,
    ) -> Result<Response<Self::GetStream>, Status> {
        println!("\tclient connected from: {:?}", request.remote_addr());

        let request_filter = request.into_inner().filter;

        let subject_filter = if let Some(BlocksFilterProto { producer, from, .. }) = &request_filter
        {
            BlocksSubjectFilter {
                producer: producer.as_ref().map(|p| p.as_str().into()).into(),
                block_height: from.as_ref().map(|bh| *bh).into(),
            }
        } else {
            BlocksSubjectFilter::default()
        };

        let maybe_to = request_filter.as_ref().and_then(|f| f.to);
        let take = request_filter
            .as_ref()
            .and_then(|f| f.take)
            .unwrap_or(u64::MAX);

        let stream = EdgeNatsClient::subscribe(subject_filter)
            .await
            .expect("All subjects must yield valid subscriptions")
            .map(|subscription_result| match subscription_result {
                Ok(nats_message) => {
                    println!("Streaming Subject:{}", &nats_message.subject);

                    Ok(BlockProto::decode(nats_message.payload).expect("must decode block"))
                }
                // TODO: Bubble up error variants appropriately
                Err(_) => Err(Status::internal("message")),
            })
            .take_while(
                move |subscription_result| match (subscription_result, maybe_to) {
                    (Ok(block_proto), Some(to_value)) => block_proto.height <= to_value,
                    (Err(_), _) => false,
                    _ => true,
                },
            )
            .take(take as usize);

        Ok(Response::new(Box::pin(stream) as Self::GetStream))
    }
}
