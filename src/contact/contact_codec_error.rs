// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Contact wire-codec failures.

use thiserror::Error;

/// Failures raised while decoding contact values from their compact wire representations.
#[derive(Debug, Error)]
pub enum ContactCodecError {
    /// A coordinate is absent where required or cannot be parsed as a decimal degree value.
    #[error("invalid coordinate")]
    InvalidCoordinate,
    /// A location string does not contain exactly one longitude/latitude separator.
    #[error("invalid location format")]
    InvalidLocation,
    /// A phone string does not match the supported local, area/local, or country/area/local form.
    #[error("invalid phone number format")]
    InvalidPhone,
    /// JSON syntax or structure prevented decoding the expected contact wire value.
    #[error("invalid contact JSON value")]
    InvalidJson(#[source] serde_json::Error),
}
