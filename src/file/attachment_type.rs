// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Attachment classifications.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// Classification of a persisted attachment.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttachmentType {
    /// Image file.
    Image,
    /// Document or unknown file.
    #[default]
    Document,
    /// Audio file.
    Audio,
    /// Video file.
    Video,
    /// vCard contact card.
    Vcard,
    /// Geographic location.
    Location,
    /// Externally hosted image.
    ExternalImage,
    /// Externally hosted audio.
    ExternalAudio,
    /// Externally hosted video.
    ExternalVideo,
}

impl AttachmentType {
    /// Returns the stable lowercase source identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Document => "document",
            Self::Audio => "audio",
            Self::Video => "video",
            Self::Vcard => "vcard",
            Self::Location => "location",
            Self::ExternalImage => "external_image",
            Self::ExternalAudio => "external_audio",
            Self::ExternalVideo => "external_video",
        }
    }

    /// Classifies a MIME content type using the source rules.
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
