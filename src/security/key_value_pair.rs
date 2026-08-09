// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! String payload entries carried by signatures.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Represents a string key and optional string value.
#[derive(Model, Redact, Clone, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct KeyValuePair {
    /// Payload key.
    pub key: String,

    /// Optional payload value.
    #[redact(level = "secret")]
    pub value: Option<String>,
}
