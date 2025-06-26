mod services;
use services::ServiceMap;

use kanal::{AsyncReceiver, AsyncSender};
use rapids::{
    codecs::BinaryCodec,
    dispatch::{RiverServer, ServiceHandler},
    types::{IncomingMessage, OutgoingMessage, ProcedureRes, RPCMetadata},
    utils,
};

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{
    Router,
    response::{IntoResponse, Response},
    routing::get,
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

    let server = Arc::new(RiverServer::new(
        BinaryCodec {},
        TestServiceHandler::new().await?,
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
    async fn new() -> anyhow::Result<Self> {
        let mut description = HashMap::new();
        description.insert(
            "adder".to_string(),
            vec![
                "add".to_string(),
                "resetCount".to_string(),
                "uploadAdd".to_string(),
                "streamAdd".to_string(),
                "subscriptionAdd".to_string(),
            ],
        );

        Ok(Self {
            description,
            service_map: ServiceMap::new().await?,
        })
    }
}

impl ServiceHandler for TestServiceHandler {
    fn description(&self) -> HashMap<String, Vec<String>> {
        self.description.clone()
    }

    async fn invoke_rpc(
        &self,
        service: String,
        procedure: String,
        metadata: RPCMetadata,
        channel: AsyncSender<OutgoingMessage>,
        payload: serde_json::Value,
        recv: AsyncReceiver<IncomingMessage>,
    ) {
        match service.as_str() {
            "adder" => {
                let service = self.service_map.adder.clone();
                tokio::spawn(async move {
                    let result = match procedure.as_str() {
                        "add" => service
                            .add(payload, &metadata)
                            .await
                            .map(ProcedureRes::Response),
                        "resetCount" => service
                            .reset_count(payload, &metadata)
                            .await
                            .map(ProcedureRes::Response),
                        "uploadAdd" => service
                            .upload_add(payload, recv, &metadata)
                            .await
                            .map(ProcedureRes::Response),
                        "streamAdd" => service
                            .stream_add(payload, recv, channel.clone(), &metadata)
                            .await
                            .map(|_| ProcedureRes::Close),
                        "subscriptionAdd" => service
                            .subscription_add(payload, channel.clone(), &metadata)
                            .await
                            .map(|_| ProcedureRes::Close),
                        _ => {
                            unreachable!(
                                "Dispatcher guarantees only correct procedures are passed along"
                            )
                        }
                    };

                    let message = match &result {
                        Ok(ProcedureRes::Response(result)) => ProcedureRes::Response(
                            serde_json::json!({ "ok": true, "payload": result }),
                        ),
                        Err(err) => ProcedureRes::Response(serde_json::json!({
                            "ok": false,
                            "payload": {"code": "UNCAUGHT_ERROR", "message": err.to_string()}
                        })),

                        Ok(ProcedureRes::Close) => ProcedureRes::Close,
                    };

                    channel
                        .send(utils::payload_to_msg(
                            message,
                            &metadata,
                            result.is_ok(),
                            result.is_err(),
                        ))
                        .await
                        .expect("TODO: handle this");
                });
            }
            _ => {
                unreachable!("Dispatcher guarantees only correct services are passed along")
            }
        }
    }
}

static DEFAULT_REPLY: &str = "(づ ◕‿◕ )づ Hello there";

async fn default_handler() -> Response {
    DEFAULT_REPLY.into_response()
}
