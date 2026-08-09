// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Categories for independently described media resources.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Presentation category assigned to a media resource.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaType {
    /// Still-image media.
    Image,
    /// Music track.
    Music,
    /// Spoken voice recording.
    Record,
    /// Generic audio content.
    Audio,
    /// Video content.
    Video,
}
