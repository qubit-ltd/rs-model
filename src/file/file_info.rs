// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Storage metadata for original files and generated file renditions.

use bigdecimal::BigDecimal;
use serde::Deserialize;
use std::path::PathBuf;

use qubit_mixin::Emptyful;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Location and technical metadata for a stored file or media rendition.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct FileInfo {
    /// ASCII path or URL interpreted by the configured storage backend.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    pub path: String,

    /// ASCII format label, such as a container or image format name.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub format: String,

    /// ASCII MIME content type of the stored bytes.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub content_type: String,

    /// Stored file size in bytes.
    pub size: i64,

    /// Image or video width in pixels, or `None` for nonvisual content.
    pub width: Option<i32>,

    /// Image or video height in pixels, or `None` for nonvisual content.
    pub height: Option<i32>,

    /// Audio or video duration in seconds, or `None` when not applicable.
    pub duration: Option<i32>,

    /// Compression quality percentage, where 100 represents lossless compression, or `None`.
    #[model(decimal(scale = 2))]
    pub quality: Option<BigDecimal>,
}

impl FileInfo {
    /// Returns whether every metadata field has its empty or absent value.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
            && self.format.is_empty()
            && self.content_type.is_empty()
            && self.size == 0
            && self.width.is_none()
            && self.height.is_none()
            && self.duration.is_none()
            && self.quality.is_none()
    }

    /// Interprets the storage path as a local filesystem path without validating it.
    #[must_use]
    pub fn to_local_path(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }

    /// Replaces both visual dimensions; `None` clears both width and height.
    pub fn set_image_size(&mut self, image_size: Option<(i32, i32)>) {
        match image_size {
            Some((width, height)) => {
                self.width = Some(width);
                self.height = Some(height);
            }
            None => {
                self.width = None;
                self.height = None;
            }
        }
    }
}

impl Emptyful for FileInfo {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}
