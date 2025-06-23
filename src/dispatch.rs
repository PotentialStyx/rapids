//! Dispatcher and WebSocket River server implementation
//!
//! # Setup
//! Please refer to the `test-server` example for how to use [`ServiceHandler`] and [`RiverServer`].
//!
//! More documentation will be written in the future.
// TODO: Real docs!!!!

use crate::{
    codecs::{Codec, DynCodec},
    types::{
        Control, HandshakeError, HandshakeRequest, HandshakeResponse, HandshakeResponseOk, Header,
        HeaderID, IPCMessage, RPCMetadata, RequestInner, RiverResult, StreamInfo,
        TransportControlMessage, TransportRequestMessage,
    },
    utils::generate_id,
};

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{
    body::Bytes,
    extract::{
        ConnectInfo,
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};

use kanal::AsyncReceiver;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

/// River Server dispatch required across all clients
pub struct RiverServer<H: ServiceHandler + Send + Sync + 'static> {
    codec: DynCodec,
    service_handler: H,
    service_description: HashMap<String, Vec<String>>,
}

/// Provides descriptions of services and executes procedure calls
pub trait ServiceHandler {
    /// Returns a [`HashMap`] that maps services to all supported procedures.
    ///
    /// This will likely only be read once and should not change.
    fn description(&self) -> HashMap<String, Vec<String>>;

    /// Responsible for invoking procedure calls,
    /// service and procedure are garunteed to be in the descriptions table.
    ///
    /// Any errors while invoking need to be handled by this method.
    ///
    /// Generally procedure calls are spawned in a background task.
    fn invoke_rpc(
        &self,
        service: String,
        procedure: String,
        metadata: RPCMetadata,
        payload: serde_json::Value,
        recv: AsyncReceiver<IPCMessage>,
    ) -> impl std::future::Future<Output = ()> + Send + Sync;
}

impl<H: ServiceHandler + Send + Sync + 'static> RiverServer<H> {
    pub fn new(codec: DynCodec, handler: H) -> Self {
        RiverServer {
            codec,
            service_description: handler.description(),
            service_handler: handler,
        }
    }

    /// Used as an [`axum`] route handler
    ///
    /// See the `test-server` example for how to use this method.
    #[allow(clippy::unused_async, reason = "Required for use as axum handler")]
    pub async fn delta(
        self: Arc<Self>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        ws: WebSocketUpgrade,
    ) -> Response {
        ws.on_upgrade(move |socket| self.on_delta_upgrade(socket, addr))
    }

    async fn on_delta_upgrade(self: Arc<Self>, mut socket: WebSocket, addr: SocketAddr) {
        info!(%addr, "New Connection");

        let client_id: String;

        if let Some(Ok(WsMessage::Binary(data))) = socket.recv().await {
            let data: TransportControlMessage = self.codec.decode_slice(&data).unwrap();
            if let Control::HandshakeRequest(HandshakeRequest {
                protocol_version,
                session_id,
                expected_session_state: _,
                metadata: _,
            }) = &data.payload
            {
                debug!(%addr, "Handshake Recieved");
                client_id = data.header.from.clone();
                info!(%addr, client_id, "Identified Client");

                let valid;
                let connection_response;

                if *protocol_version == crate::PROTOCOL_VERSION {
                    valid = true;
                    connection_response = TransportControlMessage {
                        header: Header {
                            id: generate_id(),
                            from: "SERVER".to_string(),
                            to: data.header.from,
                            seq: 0,
                            ack: 0,
                            control_flags: 0,
                            stream_id: generate_id(),
                        },
                        payload: Control::HandshakeResponse(HandshakeResponse {
                            status: RiverResult::<HandshakeResponseOk, String>::Ok(
                                HandshakeResponseOk {
                                    session_id: session_id.clone(),
                                },
                            )
                            .into(),
                        }),
                    };
                } else {
                    warn!(
                        attempted_version = %protocol_version,
                        wanted_version = %crate::PROTOCOL_VERSION,
                        client_id,
                        "Client tried to connect with incorrect version, closing connection"
                    );
                    valid = false;
                    connection_response = TransportControlMessage {
                        header: Header {
                            id: generate_id(),
                            from: "SERVER".to_string(),
                            to: data.header.from,
                            seq: 0,
                            ack: 0,
                            control_flags: 0,
                            stream_id: generate_id(),
                        },
                        payload: Control::HandshakeResponse(HandshakeResponse {
                            status: RiverResult::<HandshakeResponseOk, HandshakeError>::Err {
                                reason: format!("Expected version {}", crate::PROTOCOL_VERSION),
                                code: HandshakeError::ProtocolVersionMismatch,
                            }
                            .into(),
                        }),
                    };
                }

                socket
                    .send(WsMessage::Binary(Bytes::from_owner(
                        self.codec.encode_to_vec(&connection_response).unwrap(),
                    )))
                    .await
                    .unwrap();

                if !valid {
                    return;
                }

                debug!(%client_id, "Handshake Complete");
            } else {
                warn!("Handshake req not first message");
                socket.send(WsMessage::Close(None)).await.unwrap();
                return;
            }
        } else {
            return;
        }

        self.event_loop(socket, client_id, addr).await.unwrap();
    }

    async fn event_loop(
        self: Arc<Self>,
        mut socket: WebSocket,
        client_id: String,
        addr: SocketAddr,
    ) -> Result<()> {
        let _ = addr;
        let mut streams: HashMap<String, StreamInfo> = HashMap::new();

        let (send, recv) = kanal::unbounded_async();

        loop {
            tokio::select! {
                ws_msg = socket.recv() => {
                    let msg = match ws_msg {
                        None => break,
                        Some(msg) => msg,
                    };

                    let msg = match msg {
                        Ok(msg) => msg,
                        Err(_err) => {
                            error!("TODO: Implement WS loop error handling");
                            return Ok(());
                        },
                    };

                    match msg {
                        WsMessage::Binary(data) => {
                            let header_id: HeaderID = self.codec.decode_slice(&data).unwrap();

                            let stream_id = header_id.stream_id.clone();

                            // TODO: confirm that procedure sent has right type
                            if let Some(stream_info) = streams.get(&stream_id) {
                                let data: TransportRequestMessage = self.codec.decode_slice(&data)?;
                                if data.header.control_flags & 0b1000 == 0b1000 {
                                    stream_info.messenger.send(IPCMessage::Close).await?;
                                    streams.remove(&stream_id);
                                } else if let RequestInner::Request { payload } = data.inner {
                                    stream_info.messenger.send(IPCMessage::Request(payload)).await?;
                                } else {
                                    error!("Existing stream but init message?");
                                }
                            } else if header_id.procedure_name.is_some() && header_id.service_name.is_some() {
                                let data: TransportRequestMessage = self.codec.decode_slice(&data)?;

                                if let RequestInner::Init { payload, service_name, procedure_name } = data.inner {
                                    let (stream_send, stream_recv) = kanal::unbounded_async();

                                    // Only add stream if it is opened and not immediately closed
                                    if data.header.control_flags & 0b01010 == 0b00010 {
                                        streams.insert(header_id.stream_id, StreamInfo {
                                            stream_id: stream_id.clone(),
                                            opening_seq: header_id.seq,
                                            messenger: stream_send
                                        });
                                    }

                                    let metadata = RPCMetadata { stream_id, client_id: client_id.clone(), seq: data.header.seq, channel: send.clone() };

                                    if let Some(procedures) = self.service_description.get(&service_name) {
                                        if procedures.contains(&procedure_name) {
                                            self.service_handler.invoke_rpc(service_name, procedure_name, metadata, payload, stream_recv).await;
                                        } else {
                                            warn!(service = service_name, procedure = procedure_name, "Unknown Procedure");
                                        }
                                    } else {
                                        warn!(service = service_name, "Unknown Service");
                                    }
                                } else {
                                    error!("Non-existant stream but non-init message?");
                                }
                            } else {
                                warn!("TODO: deal with control messages?");
                            }
                        },
                        WsMessage::Close(_) => {
                            info!(client_id, "Client Disconnected");
                            for (key, entry) in streams.drain() {
                                debug!(stream_id = key, client_id, "Closing stream due to disconnect");
                                entry.messenger.send(IPCMessage::ForceClose).await?;
                            }
                            break;
                        },
                        _ => {
                            warn!(?msg, "Unknown message!");
                        }
                    }
                }
                to_send = recv.recv() => {
                    socket.send(to_send?).await?;
                }
            }
        }

        Ok(())
    }
}
