// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Classifications for content produced by AI attachment processing.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// The business purpose of an AI engine's generated content.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiResultType {
    /// Narrative description of image, audio, or video input.
    Description,
    /// Text transcribed from audio or video input.
    Transcription,
    /// Condensed summary of text or multimedia input.
    Summary,
    /// Analytical interpretation of data, text, or multimedia input.
    Analysis,
}
