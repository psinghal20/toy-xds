use xds_api::state_discovery_service_client::StateDiscoveryServiceClient;
use xds_api::{DeltaXdsRequest, DeltaXdsResponse};

pub mod xds_api {
    tonic::include_proto!("toy_xds");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = StateDiscoveryServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(DeltaXdsRequest {
        name: "Tonic".into(),
    });

    let response = client.delta_xds(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
