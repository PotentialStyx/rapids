//! Miscellaneous types used within rapids.rs

use kanal::AsyncSender;

use crate::types::RequestInner;

/// Used by the dispatcher to associate a `stream_id` with the needed metadata
pub struct StreamInfo {
    /// Channel to communicate with ongoing the procedure task
    pub messenger: AsyncSender<IncomingMessage>,
}

/// Sent from `dispatcher -> multi-message procedures`
pub enum IncomingMessage {
    /// The client has requested to close the procedure
    Close,
    /// The client has disconnected
    ForceClose,
    /// The client has sent a new message
    Request(serde_json::Value),
}

/// General information needed by procedure handlers
pub struct RPCMetadata {
    /// The `stream_id` of the invoked procedure
    pub stream_id: String,
    /// The id of the client who invoked the procedure
    pub client_id: String,
}

/// Simplified [`TransportMessage`](super::message_types::TransportMessage)
///
/// Used when communicating with the dispatcher. This enum is used because
/// the dispatcher already sets up the [`Header`](super::message_types::Header)
/// for outgoing messages.
#[allow(missing_docs)]
pub enum SimpleOutgoingMessage {
    Control(i32, super::Control),
    Request(i32, RequestInner),
}

/// Sent from `procedure -> dispatcher`
pub struct OutgoingMessage {
    /// Message data to send
    pub message: SimpleOutgoingMessage,
    /// The id of the stream that this message belongs to
    pub stream_id: String,
    /// Indicates if this is the last message of the stream
    pub close: bool,
}

/// Procedure result used by [`ServiceHandler`](crate::dispatch::ServiceHandler)
pub enum ProcedureRes {
    /// The procedure is just closing the connection
    ///
    /// This is used by `stream`/`subscription`
    Close,
    /// The procedure is sending a response message
    ///
    /// This is used by `rpc`/`upload`
    Response(serde_json::Value),
}
