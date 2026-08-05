// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! AI result classifications.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// The kind of result produced by an AI engine.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiResultType {
    /// A description of image, audio, or video content.
    Description,
    /// A textual transcription of audio or video content.
    Transcription,
    /// A summary of text, image, audio, or video content.
    Summary,
    /// An analysis of data, text, image, audio, or video content.
    Analysis,
}
