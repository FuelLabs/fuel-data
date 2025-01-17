use tokio_stream::StreamExt;

use fuel_data_protos::fuel_data_grpc_edge::streams::blocks_stream_client::BlocksStreamClient;
use fuel_data_protos::fuel_data_grpc_edge::streams::BlocksStreamRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BlocksStreamClient::connect("http://[::1]:50051").await?;

    let stream = client
        .get(BlocksStreamRequest::default())
        .await
        .unwrap()
        .into_inner();

    // stream is infinite - take just 5 elements and then disconnect
    let mut stream = stream.take(5);
    while let Some(response) = stream.next().await {
        println!("\treceived: {}", serde_json::to_string(&response.ok())?);
    }

    Ok(())
}
