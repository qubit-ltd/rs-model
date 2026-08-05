// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Upload classification vocabularies.

#[allow(unused_imports)]
use super::MediaType;

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Classification of an attachment.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttachmentType {
    /// The Image classification.
    Image,
    /// The Document classification.
    Document,
    /// The Audio classification.
    Audio,
    /// The Video classification.
    Video,
    /// The Vcard classification.
    Vcard,
    /// The Location classification.
    Location,
    /// The ExternalImage classification.
    ExternalImage,
    /// The ExternalAudio classification.
    ExternalAudio,
    /// The ExternalVideo classification.
    ExternalVideo,
}

impl AttachmentType {
    /// Classifies a MIME content type using the source model's deterministic
    /// rules.
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
