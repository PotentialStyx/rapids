#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]

//! ## ⚠️ Weird API Parts
//! #### Result Types
//! This library contains two result types used with the River
//! protocol. Information on which to use can be found in the
//! docs for [`types::result`]. While information on why there
//! are two can be found in the [`types`] page.

pub mod codecs;
pub mod dispatch;
pub mod types;
pub mod utils;

use crate::types::ProtocolVersion;

/// The protocol version currently supported by this library. At the moment this is v2.0
pub const PROTOCOL_VERSION: ProtocolVersion = ProtocolVersion::V2_0;
