mod args;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use clap::Parser;
use tokio::sync::Mutex;
use tokio_stream::{Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Code};

use xds_api::state_discovery_service_server::{StateDiscoveryService, StateDiscoveryServiceServer};
use xds_api::{DeltaXdsRequest, DeltaXdsResponse};
use args::ServerArgs;

pub mod xds_api {
    tonic::include_proto!("toy_xds");
}

#[derive(Debug, Default)]
struct ConnectedClient {
    name: String
}

#[derive(Debug, Default)]
struct ServerState {
    connected_clients: HashMap<String, ConnectedClient>
}

#[derive(Debug, Default)]
pub struct StateDiscoveryServer {
    state: Arc<Mutex<ServerState>>
}

impl ServerState {
    fn get_or_create_client(&mut self, client_name: String) -> &'_ ConnectedClient {
        let client = ConnectedClient{
            name: client_name.clone(),
        };
        self.connected_clients.entry(client_name).or_insert(client)
    }
}


// TODO: What is this async_trait macro magic?
#[tonic::async_trait]
impl StateDiscoveryService for StateDiscoveryServer {
    type DeltaXDSStream = Pin<Box<dyn Stream<Item = Result<DeltaXdsResponse, Status>> + Send + 'static>>;

    async fn delta_xds(
        &self,
        request: Request<tonic::Streaming<DeltaXdsRequest>>,
    ) -> Result<Response<Self::DeltaXDSStream>, Status> {
       let mut stream = request.into_inner();
       let state_clone = self.state.clone();
       let response_stream = async_stream::try_stream! {
            while let Some(xds_request) = stream.next().await {
                let xds_request = xds_request?;
                if let None = xds_request.node {
                    Err(Status::new(Code::InvalidArgument, "node.name is invalid"))?;
                }
                let mut state = state_clone.lock().await;
                let client = state.get_or_create_client(xds_request.node.unwrap().name);
                println!("Client Name = {:?}", client.name);
                yield DeltaXdsResponse {
                    resources: Vec::new(),
                    removed_resources: Vec::new(),
                    nonce: "test".into(),
                };
            }
       };
       Ok(Response::new(Box::pin(response_stream) as Self::DeltaXDSStream))
    }
}

// TODO: What is this error type?
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ServerArgs::parse();
    let addr = args.addr.parse::<SocketAddr>()?;
    let server = StateDiscoveryServer::default();

    Server::builder()
        .add_service(StateDiscoveryServiceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
