//! ## What does a codec do?
//! Codecs are used to transform messages into and from their
//! over the wire representation.

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// A trait that represents a codec
///
/// Codecs are used to transform messages into and from their
/// over the wire representation.
pub trait Codec: Send + Sync + Copy {
    /// Decode a slice into a value
    fn decode_slice<'a, T>(&self, v: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>;

    /// Encode a value into a vector
    fn encode_to_vec<T>(&self, value: &T) -> Result<Vec<u8>>
    where
        T: ?Sized + Serialize;
}
