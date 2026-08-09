// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Categories for independently referenced media resources.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Presentation category of a media resource.
#[derive(Model, Redact, Clone, Copy, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaType {
    /// Still-image media.
    #[default]
    Image,
    /// Music track.
    Music,
    /// Spoken voice recording.
    Record,
    /// Audio not classified as music or a voice recording.
    Audio,
    /// Video media.
    Video,
}
