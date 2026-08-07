// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Persisted AI processing results.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::ai::AiResultType;
use crate::upload::Attachment;

/// An AI engine's result for an uploaded attachment.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct AiResult {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Kind of generated result.
    #[model(index)]
    pub r#type: AiResultType,

    /// Identifier of the processed attachment.
    #[model(reference(target = Attachment, target_field = id), opaque)]
    pub attachment_id: Id,

    /// Generated content.
    pub content: String,

    /// ISO 639-1 language code for the generated content.
    #[model(index, text(min_chars = 1, max_chars = 16))]
    pub language: String,

    /// Optional ISO 639-1 language code for the source material.
    #[model(index, text(min_chars = 1, max_chars = 16))]
    pub original_language: Option<String>,

    /// AI engine name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub engine_name: String,

    /// AI engine version.
    #[model(index, text(min_chars = 1, max_chars = 64))]
    pub engine_version: String,

    /// UTC processing start timestamp with millisecond precision.
    #[model(index, time(precision = millisecond, normalization = utc))]
    pub process_start_time: DateTime<Utc>,

    /// UTC processing end timestamp with millisecond precision.
    #[model(index, time(precision = millisecond, normalization = utc))]
    pub process_end_time: DateTime<Utc>,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC soft-deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
