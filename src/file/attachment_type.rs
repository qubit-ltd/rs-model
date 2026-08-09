// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Classifications that determine how stored attachments are handled.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Category of an attachment's content or externally hosted representation.
#[derive(Model, Redact, Clone, Copy, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttachmentType {
    /// Image file stored by the platform.
    Image,
    /// Document or content type not recognized more specifically.
    #[default]
    Document,
    /// Audio file stored by the platform.
    Audio,
    /// Video file stored by the platform.
    Video,
    /// vCard contact-card file.
    Vcard,
    /// Geographic location attachment rather than file content.
    Location,
    /// Image hosted outside the platform.
    ExternalImage,
    /// Audio hosted outside the platform.
    ExternalAudio,
    /// Video hosted outside the platform.
    ExternalVideo,
}

impl AttachmentType {
    /// Returns the stable lowercase identifier used by the source-domain model.
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

    /// Maps a MIME type to a stored-file classification; unrecognized types become documents.
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
