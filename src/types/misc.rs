//! Miscellaneous types used within rapids.rs

use kanal::AsyncSender;

use crate::types::TransportMessage;

/// Used by the dispatcher to associate a `stream_id` with the needed metadata
pub struct StreamInfo {
    /// The `seq` of the initial message of the stream
    pub opening_seq: i32,
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
    /// The `seq` of the initial message of the stream
    pub seq: i32,
}

/// Sent from `procedure -> dispatcher`
pub struct OutgoingMessage {
    /// Message data to send
    pub message: TransportMessage,
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
