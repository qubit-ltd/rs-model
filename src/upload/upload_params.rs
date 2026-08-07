// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Optional parameters controlling an upload operation.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// User-supplied upload hints and hash verification material.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct UploadParams {
    /// Optional original filename.
    pub filename: Option<String>,

    /// Optional MIME content type.
    pub content_type: Option<String>,

    /// Whether the upload service removes the source file after a successful
    /// upload.
    pub delete_origin: bool,

    /// Optional hash algorithm used to verify the source.
    pub algorithm: Option<String>,

    /// Optional expected source-file hash.
    #[redact(level = "secret")]
    pub hash: Option<String>,
}
