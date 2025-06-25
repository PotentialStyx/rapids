//! Useful functions for River implementations

use nanoid::nanoid;
use tracing::debug;

use crate::types::{
    Control, OutgoingMessage, ProcedureRes, RPCMetadata, RequestInner, SimpleOutgoingMessage,
};

/// Alphanumeric alphabet used by [`generate_id`]
///
/// This is the same alphabet that is used by the [main river implementation](https://github.com/replit/river/blob/main/transport/id.ts)
pub static NANOID_ALPHABET: [char; 60] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'x', 'y', 'z', 'A', 'B', 'C',
    'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
    'X', 'Y', 'Z',
];

/// Helper function used to generate message and stream identifiers.
///
/// This function is based on the [main river implementation](https://github.com/replit/river/blob/main/transport/id.ts)
pub fn generate_id() -> String {
    nanoid!(12, &NANOID_ALPHABET)
}

/// Helper method that converts a [`ProcedureRes`] into an [`OutgoingMessage`]
///
/// The `close` parameter is used to indicate whether this message is
/// closing its stream.
///
/// The `error` parameter is used to close the stream abruptly by setting
/// the stream cancel bit. If set to true this will override the `close`
/// parameter.
pub fn payload_to_msg(
    payload: ProcedureRes,
    metadata: &RPCMetadata,
    mut close: bool,
    error: bool,
) -> OutgoingMessage {
    let mut control_flags = 0;

    if error {
        control_flags |= 0b0100;
        close = true;
    } else if close {
        control_flags |= 0b1000;
    }

    let message = match payload {
        ProcedureRes::Response(payload) => {
            // TODO: better way to log?
            debug!(
                stream_id = metadata.stream_id,
                to = metadata.client_id,
                "Sent {}",
                payload
            );

            SimpleOutgoingMessage::Request(control_flags, RequestInner::Request { payload })
        }
        ProcedureRes::Close => SimpleOutgoingMessage::Control(control_flags, Control::Close),
    };

    OutgoingMessage {
        message,
        stream_id: metadata.stream_id.clone(),
        close,
    }
}
