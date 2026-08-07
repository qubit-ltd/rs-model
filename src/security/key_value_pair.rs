// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! String payload entries carried by signatures.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// A string key and optional string value.
#[derive(Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
pub struct KeyValuePair {
    /// Payload key.
    pub key: String,

    /// Optional payload value.
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
