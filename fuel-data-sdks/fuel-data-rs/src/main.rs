mod edge;
mod errors;
mod networks;
mod streams;

use edge::FuelDataEdge;
use networks::FuelDataStreamNetwork;
use tokio_stream::StreamExt;

pub use streams::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fuel_data_edge = FuelDataEdge::connect(FuelDataStreamNetwork::Local).await?;

    let mut blocks_stream = BlocksStream::new()
        .from(0)
        .take(4)
        .stream(&fuel_data_edge)
        .await?;

    while let Some(Ok(block)) = blocks_stream.next().await {
        println!("\treceived: {}", serde_json::to_string(&block)?);
    }

    Ok(())
}
