use std::pin::Pin;
use std::time::Duration;

use tonic::{transport::Server, Request, Response, Status};

use fuel_data_protos::fuel_data_grpc_edge::streams::blocks_stream_server::{
    BlocksStream, BlocksStreamServer,
};
use fuel_data_protos::fuel_data_grpc_edge::streams::BlocksStreamRequest;
use fuel_data_protos::fuel_data_types::BlockProto;

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};

#[derive(Debug, Default)]
pub struct BlocksStreamer;

#[tonic::async_trait]
impl BlocksStream for BlocksStreamer {
    type GetStream = Pin<Box<dyn Stream<Item = Result<BlockProto, Status>> + Send>>;

    async fn get(
        &self,
        request: Request<BlocksStreamRequest>,
    ) -> Result<Response<Self::GetStream>, Status> {
        println!("EchoServer::server_streaming_echo");
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

        // creating infinite stream with requested message
        let repeat = std::iter::repeat(BlockProto::default());
        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let blocks_streamer = BlocksStreamer::default();

    Server::builder()
        .add_service(BlocksStreamServer::new(blocks_streamer))
        .serve(addr)
        .await?;

    Ok(())
}
