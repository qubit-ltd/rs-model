// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Compact presentation metadata for a media asset.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::upload::MediaType;

/// Presentation metadata for a media resource referenced independently.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct MediaInfo {
    /// Media category.
    pub r#type: MediaType,

    /// Resource size in bytes.
    pub size: i64,

    /// Display dimensions expressed as text, for example `1920x1080`.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub screen: String,

    /// Playback duration in seconds.
    pub duration: i64,
}
