// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Stored-file metadata.

use std::path::PathBuf;

use bigdecimal::BigDecimal;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// Storage metadata for a file, image, video, or audio asset.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[model(unique(name = "file_info_path", fields(path)))]
#[serde(rename_all = "camelCase")]
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
