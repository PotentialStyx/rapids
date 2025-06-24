//! Miscellaneous types used within Rapids.rs

use kanal::AsyncSender;

/// Used by the dispatcher to associate a `stream_id` with the needed metadata
pub struct StreamInfo {
    pub stream_id: String,
    pub opening_seq: i32,
    pub messenger: AsyncSender<IPCMessage>,
}

/// Sent from the dispatcher to multi-message procedures
pub enum IPCMessage {
    /// The client has requested to close the procedure
    Close,
    /// The client has disconnected
    ForceClose,
    /// The client has sent a new message
    Request(serde_json::Value),
}

/// General information needed by procedure handlers
pub struct RPCMetadata {
    pub stream_id: String,
    pub client_id: String,
    pub seq: i32,
}
