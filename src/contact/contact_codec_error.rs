// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Contact wire-codec failures.

use thiserror::Error;

/// Errors produced by contact and coordinate wire adapters.
#[derive(Debug, Error)]
pub enum ContactCodecError {
    /// A coordinate cannot be parsed as a decimal value.
    #[error("invalid coordinate")]
    InvalidCoordinate,
    /// A location does not contain exactly one longitude/latitude separator.
    #[error("invalid location format")]
    InvalidLocation,
    /// A phone does not match the supported one-, two-, or three-part format.
    #[error("invalid phone number format")]
    InvalidPhone,
    /// A JSON value cannot be decoded as the expected contact wire value.
    #[error("invalid contact JSON value")]
    InvalidJson(#[source] serde_json::Error),
}
