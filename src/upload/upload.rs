// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Persisted upload metadata.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::upload::{AttachmentType, FileInfo};

/// A file received by the upload subsystem and its generated renditions.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Upload {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Optional user-facing original filename.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub original_filename: Option<String>,

    /// Type inferred for the uploaded file.
    pub r#type: AttachmentType,

    /// Original file storage metadata.
    pub file: FileInfo,

    /// Optional screenshot rendition.
    pub screenshot: Option<FileInfo>,

    /// Optional small thumbnail rendition.
    pub small_thumbnail: Option<FileInfo>,

    /// Optional large thumbnail rendition.
    pub large_thumbnail: Option<FileInfo>,

    /// Optional hash algorithm name.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub hash_algorithm: Option<String>,

    /// Optional content hash value.
    #[model(text(min_chars = 1, max_chars = 512))]
    pub hash_value: Option<String>,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
