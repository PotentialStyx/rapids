//! # Built-in codecs
//! - JSON: [`NaiveCodec`]
//! - MessagePack: [`BinaryCodec`]
//!
//! ## What does a codec do?
//! Codecs are used to transform messages into and from their
//! over the wire representation.
//!
//! ## Implementing your own codec
//! While the [`Codec`] trait is relatively simple to implement,
//! there is no purpose of doing so at the moment.
//!
//! With how the dispatcher is currently structured, all services
//! require a [`DynCodec`], which only supports [`NaiveCodec`] and
//! [`BinaryCodec`] at the moment. However, in the future custom
//! codecs will be supported.

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[allow(clippy::missing_errors_doc)]
pub trait Codec {
    fn decode_slice<'a, T>(&self, v: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>;

    fn encode_to_vec<T>(&self, value: &T) -> Result<Vec<u8>>
    where
        T: ?Sized + Serialize;
}

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
/// # Why does this exist?
/// the [`Codec`] trait does not supporting dyn boxing, and
/// the procedure/service implementation has not been setup
/// to use generics yet while still requiring a concrete
/// [`Codec`].
#[derive(Clone, Copy)]
pub enum DynCodec {
    Binary(BinaryCodec),
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
