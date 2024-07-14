use std::collections::HashMap;

use tonic::Status;
use xds_api::state_discovery_service_client::StateDiscoveryServiceClient;
use xds_api::{DeltaXdsRequest, DeltaXdsResponse, Node};

pub mod xds_api {
    tonic::include_proto!("toy_xds");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = StateDiscoveryServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(DeltaXdsRequest {
        node: Some(Node {
            name: "toy client".into(),
        }),
        resource_names_subscribe: vec![],
        resource_names_unsubscribe: vec![],
        initial_resource_versions: HashMap::new(),
        response_nonce: "".into(),
        error_details: None,
    });

    let response = client.delta_xds(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
