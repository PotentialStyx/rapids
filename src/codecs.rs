//! This module only contains the built-in codec implementations.
//! The trait definition and more information can be found in the
//! [`types::codecs`] module.
//!
//! # Built-in codecs
//! - JSON: [`NaiveCodec`]
//! - MessagePack: [`BinaryCodec`]

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::types::Codec;

/// Basic codec that encodes messages as JSON using [`serde_json`]
#[derive(Clone, Copy)]
pub struct NaiveCodec {}

impl Codec for NaiveCodec {
    fn decode_slice<'a, T>(&self, v: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        Ok(serde_json::from_slice(v)?)
    }

    fn encode_to_vec<T>(&self, value: &T) -> Result<Vec<u8>>
    where
        T: ?Sized + Serialize,
    {
        Ok(serde_json::to_vec(value)?)
    }
}

/// Codec that encodes messages into MessagePack using [`rmp_serde`]
#[derive(Clone, Copy)]
pub struct BinaryCodec {}

impl Codec for BinaryCodec {
    fn decode_slice<'a, T>(&self, v: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        Ok(rmp_serde::from_slice(v)?)
    }

    fn encode_to_vec<T>(&self, value: &T) -> Result<Vec<u8>>
    where
        T: ?Sized + Serialize,
    {
        // This is an awful solution but rmp_serde doesn't encode enum's correctly on its own.
        // TODO: better solution? report bug to rmp_serde devs?
        let val: serde_json::Value = serde_json::from_slice(&serde_json::to_vec(value)?)?;
        Ok(rmp_serde::to_vec(&val)?)
    }
}

/// An enum that represents any built-in codec
///
/// ⚠️ This enum will likely be deprecated and removed in the
/// future.
///
/// # Why does this exist?
/// The [`Codec`] trait does not supporting dyn boxing, so this
/// enum is provided as a convenience for codec-agnostic purposes
/// that cannot use a generic.
///
/// Use of this enum is not recommended for other libraries
/// because 3rd party codec implementations don't work with
/// DynCodec.
#[derive(Clone, Copy)]
pub enum DynCodec {
    /// Binary codec
    Binary(BinaryCodec),
    /// Naive (JSON) codec
    Naive(NaiveCodec),
}

impl Codec for DynCodec {
    fn decode_slice<'a, T>(&self, v: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        match self {
            DynCodec::Binary(binary_codec) => binary_codec.decode_slice(v),
            DynCodec::Naive(naive_codec) => naive_codec.decode_slice(v),
        }
    }

    fn encode_to_vec<T>(&self, value: &T) -> Result<Vec<u8>>
    where
        T: ?Sized + Serialize,
    {
        match self {
            DynCodec::Binary(binary_codec) => binary_codec.encode_to_vec(value),
            DynCodec::Naive(naive_codec) => naive_codec.encode_to_vec(value),
        }
    }
}
