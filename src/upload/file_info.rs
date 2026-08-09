// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Technical metadata describing a stored source file or rendition.

use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Storage location and media properties for a file asset.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FileInfo {
    /// ASCII storage path or URL whose meaning depends on the configured backend.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub path: String,

    /// ASCII label for the file format.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub format: String,

    /// ASCII MIME type of the stored content.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub content_type: String,

    /// File size in bytes.
    pub size: i64,

    /// Visual width in pixels, or `None` for nonvisual content.
    pub width: Option<i32>,

    /// Visual height in pixels, or `None` for nonvisual content.
    pub height: Option<i32>,

    /// Playback length in seconds, or `None` when not applicable.
    pub duration: Option<i32>,

    /// Compression quality percentage, or `None` when unavailable.
    #[model(decimal(scale = 2))]
    pub quality: Option<BigDecimal>,
}
