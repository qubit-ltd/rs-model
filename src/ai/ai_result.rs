//! Persisted AI processing results.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::ai::AiResultType;

/// An AI engine's result for an uploaded attachment.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct AiResult {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Kind of generated result.
    pub r#type: AiResultType,
    /// Identifier of the processed attachment.
    #[model(identifier)]
    pub attachment_id: i64,
    /// Generated content.
    pub content: String,
    /// ISO 639-1 language code for the generated content.
    #[model(text(min_chars = 1, max_chars = 16))]
    pub language: String,
    /// Optional ISO 639-1 language code for the source material.
    #[model(text(min_chars = 1, max_chars = 16))]
    pub original_language: Option<String>,
    /// AI engine name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub engine_name: String,
    /// AI engine version.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub engine_version: String,
    /// UTC processing start timestamp with millisecond precision.
    #[model(time(precision = millisecond, normalization = utc))]
    pub process_start_time: DateTime<Utc>,
    /// UTC processing end timestamp with millisecond precision.
    #[model(time(precision = millisecond, normalization = utc))]
    pub process_end_time: DateTime<Utc>,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
