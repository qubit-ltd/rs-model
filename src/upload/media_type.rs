// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Upload classification vocabularies.

#[allow(unused_imports)]
use super::AttachmentType;

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Classification of independently described media.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
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
