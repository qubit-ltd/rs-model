// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Structured error information stored in system logs.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

use crate::security::KeyValuePair;

/// Error type, code, message, and optional formatting parameters.
#[derive(
    Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
#[serde(default)]
pub struct ErrorInfo {
    /// Stable error type.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub r#type: String,
    /// Stable error code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Optional user-facing error message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optional message parameters.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<KeyValuePair>>,
}
