use std::collections::HashMap;
use std::error::Error;
use std::env;
use tokio::time::{sleep, Duration};

use tonic::transport::Channel;
use tonic::Request;
use xds_api::state_discovery_service_client::StateDiscoveryServiceClient;
use xds_api::{DeltaXdsRequest, Node};

pub mod xds_api {
    tonic::include_proto!("toy_xds");
}

async fn run_delta_xds(client: &mut StateDiscoveryServiceClient<Channel>, client_name: String) -> Result<(), Box<dyn Error>> {
    let outbound = async_stream::stream! {
        loop {
            let xds_request = DeltaXdsRequest {
                node: Some(Node{name: client_name.clone()}),
                resource_names_subscribe: Vec::new(),
                resource_names_unsubscribe: Vec::new(),
                initial_resource_versions: HashMap::new(),
                error_details: None,
                response_nonce: "".into(),
            };
            sleep(Duration::from_secs(2)).await;
            yield xds_request;
        }
    };
    let response_stream = client.delta_xds(Request::new(outbound)).await?;
    let mut inbound = response_stream.into_inner();

    while let Some(xds_response) = inbound.message().await? {
        println!("DeltaXdsResponse = {:?}", xds_response)
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_name = env::var("CLIENT_NAME").unwrap_or("test-client".into());
    let mut client = StateDiscoveryServiceClient::connect("http://[::1]:50051").await?;
    run_delta_xds(&mut client, client_name).await?;
    Ok(())
}
