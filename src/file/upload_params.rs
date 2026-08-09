// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Caller-supplied hints and integrity data for an upload operation.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Optional upload metadata and digest values used to validate the source file.
#[derive(Model, Redact, Clone, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct UploadParams {
    /// Original filename, or `None` to derive it from the source path.
    #[redact(level = "secret")]
    pub filename: Option<String>,

    /// Source MIME type, or `None` when the upload service should determine it.
    pub content_type: Option<String>,

    /// Whether a successful upload should remove the source file from local storage.
    #[serde(default)]
    pub delete_origin: bool,

    /// Digest algorithm used with [`Self::hash`], or `None` to skip hash verification.
    pub algorithm: Option<String>,

    /// Expected source-file digest, or `None` to skip hash verification.
    #[redact(level = "secret")]
    pub hash: Option<String>,
}
