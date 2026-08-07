// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Optional upload-operation parameters.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// User-supplied upload hints and hash-verification material.
#[derive(Model, Redact, Clone, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct UploadParams {
    /// Optional original filename.
    #[redact(level = "secret")]
    pub filename: Option<String>,

    /// Optional MIME content type.
    pub content_type: Option<String>,

    /// Whether the upload service removes the source after success.
    #[serde(default)]
    pub delete_origin: bool,

    /// Optional hash algorithm.
    pub algorithm: Option<String>,

    /// Optional expected source-file hash.
    #[redact(level = "secret")]
    pub hash: Option<String>,
}
