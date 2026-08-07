// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Setting wire-adapter failures.

use chrono::ParseError;
use thiserror::Error;

/// Errors produced while adapting settings to or from wire representations.
#[derive(Debug, Error)]
pub enum SettingAdapterError {
    /// The supplied JSON document is malformed.
    #[error("invalid setting JSON")]
    InvalidJson(#[source] serde_json::Error),
    /// The JSON root is not an object.
    #[error("a setting JSON value must be an object")]
    InvalidJsonRoot,
    /// The source data-type name is not supported.
    #[error("invalid setting data type: {0}")]
    InvalidDataType(String),
    /// A source timestamp is not an ISO-8601 timestamp.
    #[error("invalid setting timestamp")]
    InvalidTimestamp(#[source] ParseError),
}
