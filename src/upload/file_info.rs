// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Metadata for a stored file.

use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Storage metadata for a file, image, video, or audio asset.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct FileInfo {
    /// ASCII filesystem path or storage URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
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

    /// Optional compression quality, represented as a percentage.
    pub quality: Option<BigDecimal>,
}
