use finalverse_proto::world::world_service_client::WorldServiceClient;
use finalverse_proto::world::GetWorldStateRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::args().nth(1).unwrap_or_else(|| "3003".to_string());
    let addr = format!("http://127.0.0.1:{}", port);

    println!("Connecting to {}", addr);
    let mut client = WorldServiceClient::connect(addr).await?;

    let request = tonic::Request::new(GetWorldStateRequest { region_ids: vec![] });
    let response = client.get_world_state(request).await?;

    println!("{:#?}", response.into_inner());
    Ok(())
}
