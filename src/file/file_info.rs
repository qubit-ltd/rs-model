// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Stored-file metadata.

use bigdecimal::BigDecimal;
use serde::Deserialize;
use std::path::PathBuf;

use qubit_mixin::Emptyful;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Storage metadata for a file, image, video, or audio asset.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
#[model(unique(name = "file_info_path", fields(path), ignore_case(path)))]
pub struct FileInfo {
    /// ASCII filesystem path or storage URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    pub path: String,

    /// ASCII format name.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub format: String,

    /// ASCII MIME content type.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub content_type: String,

    /// File size in bytes.
    pub size: i64,

    /// Optional image or video width in pixels.
    pub width: Option<i32>,

    /// Optional image or video height in pixels.
    pub height: Option<i32>,

    /// Optional audio or video duration in seconds.
    pub duration: Option<i32>,

    /// Optional compression quality percentage with two fractional digits.
    #[model(decimal(scale = 2))]
    pub quality: Option<BigDecimal>,
}

impl FileInfo {
    /// Returns whether the storage path is empty.
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

    /// Returns this object's local filesystem path.
    #[must_use]
    pub fn to_local_path(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }

    /// Replaces the optional image dimensions.
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
