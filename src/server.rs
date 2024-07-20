mod args;

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use clap::Parser;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::{Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};

use args::ServerArgs;
use xds_api::state_discovery_service_server::{StateDiscoveryService, StateDiscoveryServiceServer};
use xds_api::{DeltaXdsRequest, DeltaXdsResponse};

pub mod xds_api {
    tonic::include_proto!("toy_xds");
}

#[derive(Debug)]
pub struct StateDiscoveryServer {
    cache: Arc<Mutex<ClientXdsCache>>,
}

fn handle_incoming_request(req: DeltaXdsRequest) -> Result<(), Box<dyn Error>> {
    // TODO: Add handler for taking incoming request and store the client details
    // and requested resources.
    return Ok(());
}

// TODO: What is this async_trait macro magic?
#[tonic::async_trait]
impl StateDiscoveryService for StateDiscoveryServer {
    type DeltaXDSStream =
        Pin<Box<dyn Stream<Item = Result<DeltaXdsResponse, Status>> + Send + 'static>>;

    async fn delta_xds(
        &self,
        request: Request<tonic::Streaming<DeltaXdsRequest>>,
    ) -> Result<Response<Self::DeltaXDSStream>, Status> {
        let cache = self.cache.clone();
        // TODO: Get the client name from the metadata header
        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);
        {
            let mut cache_guard = cache.lock().await;
            cache_guard.add_new_client("test-client".into(), tx);
            println!("Cache: {:?}", cache_guard);
        }

        // TODO: Setup the client, tx and resource list cache
        tokio::spawn(async move {
            while let Some(xds_request) = stream.next().await {
                if let Ok(xds_request) = xds_request {
                    if let Err(err) = handle_incoming_request(xds_request) {
                        eprintln!("failed handling incoming xds request {}", err);
                    }
                }
            }
        });

        let response_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(response_stream) as Self::DeltaXDSStream
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ServerArgs::parse();
    let addr = args.addr.parse::<SocketAddr>()?;
    let client_cache = Arc::new(Mutex::new(ClientXdsCache::new()));
    let server = StateDiscoveryServer{
        cache: client_cache.clone(),
    };
    Server::builder()
        .add_service(StateDiscoveryServiceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug)]
struct ClientCacheEntry {
    tx: Sender<Result<DeltaXdsResponse, Status>>,
    resources_subscribed: Option<Vec<String>>
}

#[derive(Debug)]
struct ClientXdsCache {
    clients: HashMap<String, ClientCacheEntry>
}

impl ClientXdsCache {
    fn new() -> Self {
        ClientXdsCache{
            clients: HashMap::new()
        }
    }

    fn add_new_client(&mut self, client_name: String, tx: Sender<Result<DeltaXdsResponse, Status>>) {
        self.clients.insert(client_name, ClientCacheEntry{
            tx: tx,
            resources_subscribed: None
        });
    }
}
