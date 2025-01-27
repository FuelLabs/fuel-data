mod streamers;

use streamers::BlocksStreamer;
use tonic::transport::Server;

use fuel_data_protos::fuel_data_grpc_edge::streams::blocks_stream_server::BlocksStreamServer;

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
