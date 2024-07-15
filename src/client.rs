use std::collections::HashMap;
use std::error::Error;

use tonic::transport::Channel;
use tonic::{Request};
use xds_api::state_discovery_service_client::StateDiscoveryServiceClient;
use xds_api::{DeltaXdsRequest, Node};

pub mod xds_api {
    tonic::include_proto!("toy_xds");
}

async fn run_delta_xds(client: &mut StateDiscoveryServiceClient<Channel>) -> Result<(), Box<dyn Error>> {
    let outbound = async_stream::stream! {
        let xds_request = DeltaXdsRequest {
            node: Some(Node{name: "test-client".into()}),
            resource_names_subscribe: Vec::new(),
            resource_names_unsubscribe: Vec::new(),
            initial_resource_versions: HashMap::new(),
            error_details: None,
            response_nonce: "".into(),
        };
        yield xds_request;
    };
    let response_stream = client.delta_xds(Request::new(outbound)).await?;
    let mut inbound = response_stream.into_inner();

    while let Some(xdsResponse) = inbound.message().await? {
        println!("DeltaXdsResponse = {:?}", xdsResponse)
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = StateDiscoveryServiceClient::connect("http://[::1]:50051").await?;
    run_delta_xds(&mut client).await?;
    Ok(())
}
