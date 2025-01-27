mod streamers;

use std::time::Duration;

use streamers::BlocksStreamer;
use tonic::transport::Server;

use fuel_data_protos::fuel_data_grpc_edge::streams::blocks_stream_server::BlocksStreamServer;

use async_nats::ConnectOptions;

pub struct EdgeNatsClient {
    pub client: async_nats::Client,
}

impl EdgeNatsClient {
    pub async fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        let nats_url = fuel_data_cluster::where_is::relay_nats();
        let user = "default_user".to_owned();
        let password = "".to_owned();

        let client = ConnectOptions::with_user_and_password(user, password)
            .connection_timeout(Duration::from_secs(5))
            .max_reconnects(1)
            .connect(nats_url)
            .await?;

        Ok(Self { client })
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
