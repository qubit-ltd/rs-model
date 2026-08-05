// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Optional upload-operation parameters.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// User-supplied upload hints and hash-verification material.
#[derive(Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
pub struct UploadParams {
    /// Optional original filename.
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Optional MIME content type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Whether the upload service removes the source after success.
    #[serde(default)]
    pub delete_origin: bool,
    /// Optional hash algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// Optional expected source-file hash.
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}
