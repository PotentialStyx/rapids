mod services;
use services::ServiceMap;

use kanal::AsyncReceiver;
use rapids_rs::{
    codecs::{BinaryCodec, DynCodec},
    dispatch::{RiverServer, ServiceHandler},
    types::RPCMetadata,
};

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string())
        .parse()?;

    let codec = DynCodec::Binary(BinaryCodec {});

    let server = Arc::new(RiverServer::new(
        codec,
        TestServiceHandler::new(codec).await?,
    ));

    let app = Router::new()
        .route("/delta", get(|addr, ws| server.delta(addr, ws)))
        .fallback(get(default_handler));
    info!("River server flowing at: ws://{}/delta", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

struct TestServiceHandler {
    description: HashMap<String, Vec<String>>,
    service_map: services::ServiceMap,
}

impl TestServiceHandler {
    async fn new(codec: DynCodec) -> anyhow::Result<Self> {
        let mut description = HashMap::new();
        description.insert(
            "example".to_string(),
            vec![
                "add".to_string(),
                "resetCount".to_string(),
                "streamAdd".to_string(),
            ],
        );

        Ok(Self {
            description,
            service_map: ServiceMap::new(codec).await?,
        })
    }
}

impl ServiceHandler for TestServiceHandler {
    fn description(&self) -> HashMap<String, Vec<String>> {
        self.description.clone()
    }

    // TODO: proper error handling
    async fn invoke_rpc(
        &self,
        service: String,
        procedure: String,
        metadata: RPCMetadata,
        payload: serde_json::Value,
        recv: AsyncReceiver<rapids_rs::types::IPCMessage>,
    ) {
        match service.as_str() {
            "example" => {
                let service = self.service_map.example.clone();
                match procedure.as_str() {
                    "add" => {
                        tokio::spawn(async move {
                            service.add(payload, metadata).await.unwrap();
                        });
                    }
                    "resetCount" => {
                        tokio::spawn(async move {
                            service.reset_count(payload, metadata).await.unwrap();
                        });
                    }
                    "streamAdd" => {
                        tokio::spawn(async move {
                            service.stream_add(payload, recv, metadata).await.unwrap();
                        });
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
}

static DEFAULT_REPLY: &str = "(づ ◕‿◕ )づ Hello there";

async fn default_handler() -> Response {
    DEFAULT_REPLY.into_response()
}
