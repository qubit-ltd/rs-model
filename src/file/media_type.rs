// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Media classifications.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// Classification of independently described media.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
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
