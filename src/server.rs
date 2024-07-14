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
    async fn delta_xds(
        &self,
        request: Request<DeltaXdsRequest>,
    ) -> Result<Response<DeltaXdsResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = DeltaXdsResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
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
