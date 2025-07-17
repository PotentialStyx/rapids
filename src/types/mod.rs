//! # River implementation types
//! Types have been translated as nicely as possible to rust,
//! while still behaving correctly with serde. This however does
//! result in some types being unergonomic. In the future this
//! might be fixed with two seperate sets of types, one set for
//! use internally with serde and another set for use to implement
//! services/clients.
//!
//! ## ⚠️ RiverResult
//! One resuly of these type translations is having both
//! [`RiverResult`] and [`RiverResultInternal`]. Types will require
//! [`RiverResultInternal`] to be used, but you will need to make
//! it first as a normal [`RiverResult`]. To understand how to
//! properly use the two types, read the [`result`] page.

pub mod codecs;
pub mod control;
pub mod message_types;
pub mod misc;
pub mod result;

pub use codecs::*;
pub use control::*;
pub use message_types::*;
pub use misc::*;
pub use result::*;
