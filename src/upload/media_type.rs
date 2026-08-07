// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Upload classification vocabularies.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Classification of independently described media.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaType {
    /// The Image classification.
    Image,
    /// The Music classification.
    Music,
    /// The Record classification.
    Record,
    /// The Audio classification.
    Audio,
    /// The Video classification.
    Video,
}
