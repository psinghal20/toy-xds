use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};

use xds_api::state_discovery_service_server::{StateDiscoveryService, StateDiscoveryServiceServer};
use xds_api::{DeltaXdsRequest, DeltaXdsResponse};

pub mod xds_api {
    tonic::include_proto!("toy_xds");
}

#[derive(Debug, Default)]
pub struct StateDiscoveryServer {}


// TODO: What is this async_trait macro magic?
#[tonic::async_trait]
impl StateDiscoveryService for StateDiscoveryServer {
    type DeltaXDSStream = Pin<Box<dyn Stream<Item = Result<DeltaXdsResponse, Status>> + Send + 'static>>;

    async fn delta_xds(
        &self,
        request: Request<tonic::Streaming<DeltaXdsRequest>>,
    ) -> Result<Response<Self::DeltaXDSStream>, Status> {
       let mut stream = request.into_inner();
       let response_stream = async_stream::try_stream! {
            while let Some(xds_request) = stream.next().await {
                let xds_request = xds_request?;
                println!("Request = {:?}", xds_request);
                yield DeltaXdsResponse {
                    resources: Vec::new(),
                    removed_resources: Vec::new(),
                    nonce: "test".into(),
                }
            }
       };
       Ok(Response::new(Box::pin(response_stream) as Self::DeltaXDSStream))
    }
}

// TODO: What is this error type?
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let server = StateDiscoveryServer::default();

    Server::builder()
        .add_service(StateDiscoveryServiceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
