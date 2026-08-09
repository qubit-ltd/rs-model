// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Optional caller input that controls an upload and its hash validation.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Caller-provided filename, MIME type, cleanup, and digest-validation options.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct UploadParams {
    /// Original filename, or `None` when the service should infer it.
    pub filename: Option<String>,

    /// Source MIME type, or `None` when the service should determine it.
    pub content_type: Option<String>,

    /// Whether the service removes the local source file after a successful upload.
    pub delete_origin: bool,

    /// Digest algorithm used with [`Self::hash`], or `None` to omit validation.
    pub algorithm: Option<String>,

    /// Expected source-file digest, or `None` to omit validation.
    #[redact(level = "secret")]
    pub hash: Option<String>,
}
