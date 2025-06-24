#![allow(missing_docs)] // TODO: finish documentation and remove this

//! Full River message representations
//!
//! # Why are there multiple transport message structs?
//! Due to how the River protocol was written, the payload
//! can either have a tagged type (like with control messages),
//! or be completely protocol defined (which sometimes
//! still includes a type tag).
//!
//! This makes it hard to represent all messages with the
//! same struct that serde can properly interpret, hence
//! multiple structs that the dispatcher can figure out
//! which to use.
//!
//! # Which transport message type should I use?
//! [`TransportMessage`] is likely what you will want as it
//! allows dealing with both [`TransportControlMessage`] and
//! [`TransportRequestMessage`]. It is also the type sent
//! from `procedure -> dispatch`

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Shared header information that all messages have
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub id: String,
    pub from: String,
    pub to: String,
    pub seq: i32,
    pub ack: i32,
    pub stream_id: String,
    pub control_flags: i32,
}

/// Minimal representation of [`Header`] required for the dispatcher
///
/// The dispatcher uses this information to figure out which
/// `TransportMessage` struct it should use when decoding a
/// message.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderID {
    pub stream_id: String,
    pub service_name: Option<String>,
    pub procedure_name: Option<String>,
    pub control_flags: i32,
    pub seq: i32,
    pub ack: i32,
}

/// Generic transport message
///
/// See [`TransportControlMessage`], [`TransportRequestMessage`], and
/// [`message_types`](super::message_types).
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TransportMessage {
    Control(TransportControlMessage),
    Request(TransportRequestMessage),
}

/// Full `Control` message representation
///
/// Control messages are used for handshakes, heartbeats,
/// and stream closures throughout the entire session.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransportControlMessage {
    #[serde(flatten)]
    pub header: Header,

    pub payload: super::Control,
}

/// Full `Init`/`Request`/`Result` message representation
///
/// This message type is used for all non-control messages
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransportRequestMessage {
    #[serde(flatten)]
    pub header: Header,

    /// Using an inner, flattened, enum is used to make
    /// coding the dispatcher easier
    #[serde(flatten)]
    pub inner: RequestInner,
}

/// Used by [`TransportRequestMessage`] to simplify logic within
/// the dispatcher
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all_fields = "camelCase", untagged)]
pub enum RequestInner {
    /// Used to initialize a new procedure
    Init {
        service_name: String,
        procedure_name: String,
        payload: Value,
    },
    /// Used to sent updates/responses to an ongoing procedure
    Request { payload: Value },
}
