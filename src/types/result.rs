//! # Result Types
//!
//! ## Why are there two result types?
//! Read the documention for the [`types`](`super`) module.
//!
//! ## Which should I use?
//! You should try and only deal with [`RiverResult`], however api
//! structs require [`RiverResultInternal`]. To solve this just use
//! [`Into::into`], which is implemented for [`RiverResult`] into
//! [`RiverResultInternal`].
//!
//! If you recieve a [`RiverResultInternal`], [`TryFrom::try_from`]
//! is implemented and should get you a [`RiverResult`], while
//! technically it can fail, this can only occur if the sender sent
//! an invalid result.

use std::fmt::Display;

use anyhow::format_err;
use serde::{Deserialize, Serialize};

/// Result type used by the River protocol.
///
/// To serialize this enum, or use it in a message it needs to be
/// converted to a [`RiverResultInternal`] using [`Into::into`].
///
/// The reason for why this is needed is given in the documentation
/// page for the [`result`](super::result) module.
#[derive(Clone, Debug)]
pub enum RiverResult<T, E: ToString> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err {
        /// The associated error message
        message: String,
        /// The error code
        code: E,
    },
}

impl<T, E: ToString> RiverResult<T, E> {
    /// Returns `true` if the result is [`Ok`](RiverResult::Ok).
    #[must_use]
    pub const fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(..))
    }

    /// Returns `true` if the result is [`Err`](RiverResult::Err).
    #[must_use]
    pub const fn is_err(&self) -> bool {
        matches!(self, Self::Err { .. })
    }
}

impl<T, E: TryFrom<String, Error = E2> + ToString, E2: Display> TryFrom<RiverResultInternal<T>>
    for RiverResult<T, E>
{
    type Error = anyhow::Error;

    fn try_from(result: RiverResultInternal<T>) -> Result<Self, Self::Error> {
        if result.ok {
            if let Some(inner) = result.inner {
                Ok(RiverResult::Ok(inner))
            } else {
                Err(format_err!("Expected inner to be Some when ok is true"))
            }
        } else {
            if let Some(code) = result.code {
                if let Some(message) = result.message {
                    return Ok(RiverResult::Err {
                        code: code.try_into().map_err(|e| format_err!("{e}"))?,
                        message,
                    });
                }
            }

            Err(format_err!(
                "Expected code and reason to be Some when ok is false"
            ))
        }
    }
}

/// Internal, and serde capable representation of [`RiverResult`]
///
/// To use the data held within this struct convert it into a
/// [`RiverResult`], using [`TryFrom::try_from`].
///
/// The reason for why this is needed is given in the documentation
/// page for the [`result`](super::result) module.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RiverResultInternal<T> {
    ok: bool,
    message: Option<String>,
    code: Option<String>,
    #[serde(flatten)]
    inner: Option<T>,
}

impl<T, E: ToString> From<RiverResult<T, E>> for RiverResultInternal<T> {
    fn from(result: RiverResult<T, E>) -> Self {
        match result {
            RiverResult::Ok(val) => RiverResultInternal {
                ok: true,
                message: None,
                code: None,
                inner: Some(val),
            },
            RiverResult::Err {
                message: reason,
                code,
            } => RiverResultInternal {
                ok: false,
                message: Some(reason),
                code: Some(code.to_string()),
                inner: None,
            },
        }
    }
}
