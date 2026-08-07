// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Compact media metadata.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::MediaType;

/// Metadata describing an independently referenced media resource.
#[derive(Model, Redact, Clone, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct MediaInfo {
    /// Media classification.
    pub r#type: MediaType,

    /// Size in bytes.
    pub size: i64,

    /// Screen dimensions such as `1920x1080`.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub screen: String,

    /// Duration in seconds.
    pub duration: i64,
}
