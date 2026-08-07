// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Media classifications.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Classification of independently described media.
#[derive(Model, Redact, Clone, Copy, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaType {
    /// Image media.
    #[default]
    Image,
    /// Music media.
    Music,
    /// Voice recording.
    Record,
    /// Generic audio media.
    Audio,
    /// Video media.
    Video,
}
