// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Metadata for an uploaded source file and its optional derived images.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::upload::AttachmentType;
use crate::upload::FileInfo;

/// A stored uploaded file and the optional renditions produced from it.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Upload {
    /// Database identifier; the default value denotes an unpersisted upload.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Original filename shown to users, or `None` when it was not captured.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub original_filename: Option<String>,

    /// Attachment classification inferred for the original file.
    pub r#type: AttachmentType,

    /// Storage metadata for the original uploaded file.
    pub file: FileInfo,

    /// Screenshot rendition, or `None` when it was not generated.
    pub screenshot: Option<FileInfo>,

    /// Small thumbnail rendition, or `None` when it was not generated.
    pub small_thumbnail: Option<FileInfo>,

    /// Large thumbnail rendition, or `None` when it was not generated.
    pub large_thumbnail: Option<FileInfo>,

    /// Digest algorithm name, or `None` when no digest is stored.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub hash_algorithm: Option<String>,

    /// Source-file digest, or `None` when integrity data is unavailable.
    #[model(text(min_chars = 1, max_chars = 512))]
    pub hash_value: Option<String>,

    /// UTC creation instant, rounded to seconds.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC soft-deletion instant, or `None` while the upload is retained.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
