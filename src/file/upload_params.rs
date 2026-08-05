// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Optional upload-operation parameters.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// User-supplied upload hints and hash-verification material.
#[derive(Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadParams {
    /// Optional original filename.
    #[redact(level = "secret")]
    pub filename: Option<String>,
    /// Optional MIME content type.
    pub content_type: Option<String>,
    /// Whether the upload service removes the source after success.
    pub delete_origin: bool,
    /// Optional hash algorithm.
    pub algorithm: Option<String>,
    /// Optional expected source-file hash.
    #[redact(level = "secret")]
    pub hash: Option<String>,
}
