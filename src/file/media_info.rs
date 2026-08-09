// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Compact metadata for independently referenced media resources.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::MediaType;

/// Presentation metadata for a media resource referenced independently of uploads.
#[derive(Model, Redact, Clone, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct MediaInfo {
    /// Media category used by the consumer.
    pub r#type: MediaType,

    /// Resource size in bytes.
    pub size: i64,

    /// Display dimensions encoded as text, such as `1920x1080`.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub screen: String,

    /// Playback duration in seconds.
    pub duration: i64,
}
