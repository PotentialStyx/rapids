//! Non-RPC messages that can be sent in either direction to communicate handshakes, heartbeats, and stream closures.

use anyhow::format_err;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::types::RiverResultInternal;

/// The payload for [`TransportControlMessage`](super::message_types::TransportControlMessage)
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all_fields = "camelCase", tag = "type")]
pub enum Control {
    /// Used to close procedures (upload, stream, download), can be sent by either client or server
    #[serde(rename = "CLOSE")]
    Close,
    /// Used for heartbeat messages
    #[serde(rename = "ACK")]
    Ack,
    /// Initial message sent from <strong>`client -> server`</strong>
    #[serde(rename = "HANDSHAKE_REQ")]
    HandshakeRequest(HandshakeRequest),
    /// Initial response sent from <strong>`server -> client`</strong>
    #[serde(rename = "HANDSHAKE_RESP")]
    HandshakeResponse(HandshakeResponse),
}

/// Initial message sent from <strong>`client -> server`</strong> when the connection is opened.
///
/// This must be the first message sent by the client, and cannot be recieved again after.
/// Once the server recieves this message it responds with [`HandshakeResponse`]
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HandshakeRequest {
    pub protocol_version: ProtocolVersion,
    pub session_id: String,
    pub expected_session_state: ExpectedSessionState,
    pub metadata: Option<serde_json::Value>,
}

/// First message sent from <strong>`server -> client`</strong> when connection is opened.
///
/// This must be sent in response to [`HandshakeRequest`] to alert the client whether the handshake was successful.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HandshakeResponse {
    /// Use [`RiverResult<HandshakeResponseOk, HandshakeError>`](super::result::RiverResult) and [`Into::into`] to construct this field
    pub status: RiverResultInternal<HandshakeResponseOk>,
}

/// When a client has sent a valid handshake, their `session_id` is sent back.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HandshakeResponseOk {
    pub session_id: String,
}

/// Errors that the server can alert the client of when a handshake fails.
///
/// All of these errors are fatal besides [`SessionStateMismatch`](HandshakeError::SessionStateMismatch),
/// which the client will likely retry if it recieves.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HandshakeError {
    /// Retriable
    SessionStateMismatch,
    /// ⚠️ Fatal
    MalformedHandshakeMeta,
    /// ⚠️ Fatal
    MalformedHandshake,
    /// ⚠️ Fatal
    ProtocolVersionMismatch,
    /// ⚠️ Fatal
    RejectedByCustomHandler,
}

impl Display for HandshakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            HandshakeError::SessionStateMismatch => "SESSION_STATE_MISMATCH",
            HandshakeError::MalformedHandshakeMeta => "MALFORMED_HANDSHAKE_META",
            HandshakeError::MalformedHandshake => "MALFORMED_HANDSHAKE",
            HandshakeError::ProtocolVersionMismatch => "PROTOCOL_VERSION_MISMATCH",
            HandshakeError::RejectedByCustomHandler => "REJECTED_BY_CUSTOM_HANDLER",
        };

        f.write_str(to_write)
    }
}

impl TryFrom<String> for HandshakeError {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value: &str = &value;
        match value {
            "SESSION_STATE_MISMATCH" => Ok(HandshakeError::SessionStateMismatch),
            "MALFORMED_HANDSHAKE_META" => Ok(HandshakeError::MalformedHandshakeMeta),
            "MALFORMED_HANDSHAKE" => Ok(HandshakeError::MalformedHandshake),
            "PROTOCOL_VERSION_MISMATCH" => Ok(HandshakeError::ProtocolVersionMismatch),
            "REJECTED_BY_CUSTOM_HANDLER" => Ok(HandshakeError::RejectedByCustomHandler),
            _ => Err(format_err!("Unknown HandshakeError: `{value}`")),
        }
    }
}

/// River protocol version info
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum ProtocolVersion {
    /// # v0
    /// Unsupported
    #[serde(rename = "v0")]
    V0,
    /// # v1
    /// Unsupported
    #[serde(rename = "v1")]
    V1,
    /// # v1.0
    /// Unsupported
    #[serde(rename = "v1.1")]
    V1_1,
    /// # v2.0
    /// The only version this library currently supports
    #[serde(rename = "v2.0")]
    V2_0,
    /// # Unknown version
    /// Unsupported
    #[serde(untagged)]
    Unknown(String),
}

impl Display for ProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            ProtocolVersion::V0 => "v0",
            ProtocolVersion::V1 => "v1",
            ProtocolVersion::V1_1 => "v1.1",
            ProtocolVersion::V2_0 => "v2.0",
            ProtocolVersion::Unknown(version) => version.as_str(),
        };
        f.write_str(to_write)
    }
}

/// Session state used for transparent reconnects
///
/// NOTE: ⚠️ Transparent reconnects are not supported by the built in dispatcher/server. ⚠️
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpectedSessionState {
    pub next_expected_seq: i64,
    pub next_sent_seq: i64,
}
