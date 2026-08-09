// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Content classifications used for upload attachments.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Content or hosting classification for an attachment.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttachmentType {
    /// Image content stored by the platform.
    Image,
    /// Document or unrecognized content.
    Document,
    /// Audio content stored by the platform.
    Audio,
    /// Video content stored by the platform.
    Video,
    /// vCard contact information.
    Vcard,
    /// Geographic location content.
    Location,
    /// Externally hosted image.
    ExternalImage,
    /// Externally hosted audio.
    ExternalAudio,
    /// Externally hosted video.
    ExternalVideo,
}

impl AttachmentType {
    /// Maps a MIME type to an attachment type; unrecognized types become documents.
    #[must_use]
    pub fn for_content_type(content_type: &str) -> Self {
        if content_type.starts_with("image/") {
            Self::Image
        } else if content_type.starts_with("audio/") {
            Self::Audio
        } else if content_type.starts_with("video/") {
            Self::Video
        } else if content_type == "text/x-vcard" {
            Self::Vcard
        } else {
            Self::Document
        }
    }
}
