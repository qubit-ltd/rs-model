//! Uploaded-file and attachment domain models.

mod attachment;
mod file_info;
mod media_info;
#[allow(clippy::module_inception)]
mod upload;
mod upload_params;

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

pub use attachment::Attachment;
pub use file_info::FileInfo;
pub use media_info::MediaInfo;
pub use upload::Upload;
pub use upload_params::UploadParams;

/// Classification of an attachment.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttachmentType {
    Image,
    Document,
    Audio,
    Video,
    Vcard,
    Location,
    ExternalImage,
    ExternalAudio,
    ExternalVideo,
}

impl AttachmentType {
    /// Classifies a MIME content type using the source model's deterministic rules.
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

/// Classification of independently described media.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaType {
    Image,
    Music,
    Record,
    Audio,
    Video,
}
