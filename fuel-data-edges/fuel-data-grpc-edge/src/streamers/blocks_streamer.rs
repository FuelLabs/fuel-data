use std::pin::Pin;

use tonic::{Request, Response, Status};

use fuel_data_protos::fuel_data_grpc_edge::streams::blocks_stream_server::BlocksStream;
use fuel_data_protos::fuel_data_grpc_edge::streams::BlocksStreamRequest;
use fuel_data_protos::fuel_data_types::BlockProto;

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};

use prost::Message;

use crate::EdgeNatsClient;

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

        let filter = request.into_inner().filter;

        let subject = filter
            .map(|filter| {
                let from_block_height = filter
                    .from
                    .map(|f| f.to_string())
                    .unwrap_or("*".to_string());
                let producer = filter
                    .producer
                    .map(|f| f.to_string())
                    .unwrap_or("*".to_string());

                format!("blocks.{}.{}", from_block_height, producer)
            })
            .unwrap_or("blocks.*.*".to_string());
        println!("\trequested subject: {}", subject);

        let nats_client = EdgeNatsClient::connect()
            .await
            .expect("NATS Client connection failed");

        let mut subscription = nats_client
            .client
            .subscribe(subject)
            .await
            .expect("All subjects must yield valid subscriptions");

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = subscription.next().await {
                println!("\tBlock received");
                println!("Subject{}", &item.subject);
                let block = BlockProto::decode(item.payload).expect("must decode block");
                println!("Block{}", &(serde_json::to_string(&block).unwrap()));

                match tx.send(Result::<_, Status>::Ok(block)).await {
                    Ok(_) => {
                        // item (server response) was queued to be send to client
                    }
                    Err(_item) => {
                        // output_stream was built from rx and both are dropped
                        break;
                    }
                }
            }
            println!("\tclient disconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output_stream) as Self::GetStream))
    }
}
