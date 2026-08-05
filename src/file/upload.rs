// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Persisted upload metadata and generated renditions.

use std::path::Path;

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{AttachmentType, FileInfo, UploadParams};

/// A received file and its generated image renditions.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Optional user-facing original filename.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    #[redact(level = "secret")]
    pub original_filename: Option<String>,
    /// Type inferred for the uploaded file.
    #[model(index)]
    pub r#type: AttachmentType,
    /// Original file storage metadata.
    #[redact(nested)]
    pub file: FileInfo,
    /// Optional screenshot rendition.
    #[redact(nested)]
    pub screenshot: Option<FileInfo>,
    /// Optional small-thumbnail rendition.
    #[redact(nested)]
    pub small_thumbnail: Option<FileInfo>,
    /// Optional large-thumbnail rendition.
    #[redact(nested)]
    pub large_thumbnail: Option<FileInfo>,
    /// Optional hash algorithm name.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub hash_algorithm: Option<String>,
    /// Optional content hash value.
    #[model(text(min_chars = 1, max_chars = 512))]
    #[redact(level = "secret")]
    pub hash_value: Option<String>,
    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,
    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Upload {
    /// Suffix appended to screenshot renditions.
    pub const SCREENSHOT_SUFFIX: &'static str = "_screenshot";
    /// Suffix appended to small thumbnails.
    pub const SMALL_THUMBNAIL_SUFFIX: &'static str = "_small";
    /// Suffix appended to large thumbnails.
    pub const LARGE_THUMBNAIL_SUFFIX: &'static str = "_large";
    /// Generated image extension.
    pub const IMAGE_EXTENSION: &'static str = "jpg";
    /// Generated image format.
    pub const IMAGE_FORMAT: &'static str = "JPEG";
    /// Generated image MIME type.
    pub const IMAGE_CONTENT_TYPE: &'static str = "image/jpeg";

    /// Creates upload metadata for an existing local file.
    ///
    /// The original filename falls back to the path's filename and an absent
    /// content type falls back to `application/octet-stream`.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when the source file metadata cannot be read or a
    /// relative path cannot be made absolute.
    pub fn create(path: &Path, params: &UploadParams) -> std::io::Result<Self> {
        let content_type = params
            .content_type
            .as_deref()
            .unwrap_or("application/octet-stream");
        let mut upload = Self {
            original_filename: params.filename.clone().or_else(|| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .map(str::to_owned)
            }),
            hash_algorithm: params.algorithm.clone(),
            hash_value: params.hash.clone(),
            ..Self::default()
        };
        upload.set_file_info(path, content_type)?;
        upload.set_content_type(content_type);
        Ok(upload)
    }

    /// Replaces the original-file metadata from a local path.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when the source file metadata cannot be read or a
    /// relative path cannot be made absolute.
    pub fn set_file_info(&mut self, path: &Path, content_type: &str) -> std::io::Result<&FileInfo> {
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };
        self.file = FileInfo {
            path: absolute_path.to_string_lossy().into_owned(),
            content_type: content_type.to_owned(),
            size: std::fs::metadata(path)?
                .len()
                .try_into()
                .unwrap_or(i64::MAX),
            ..FileInfo::default()
        };
        Ok(&self.file)
    }

    /// Updates the original file MIME type and inferred attachment type.
    pub fn set_content_type(&mut self, content_type: &str) -> AttachmentType {
        self.file.content_type = content_type.to_owned();
        self.r#type = AttachmentType::for_content_type(content_type);
        self.r#type
    }

    /// Creates and stores screenshot metadata in the temporary directory.
    pub fn set_screenshot_info(&mut self) -> &FileInfo {
        self.screenshot = Some(self.rendition(Self::SCREENSHOT_SUFFIX));
        self.screenshot
            .as_ref()
            .expect("screenshot was just stored")
    }

    /// Creates and stores small-thumbnail metadata in the temporary directory.
    pub fn set_small_thumbnail_info(&mut self) -> &FileInfo {
        self.small_thumbnail = Some(self.rendition(Self::SMALL_THUMBNAIL_SUFFIX));
        self.small_thumbnail
            .as_ref()
            .expect("small thumbnail was just stored")
    }

    /// Creates and stores large-thumbnail metadata in the temporary directory.
    pub fn set_large_thumbnail_info(&mut self) -> &FileInfo {
        self.large_thumbnail = Some(self.rendition(Self::LARGE_THUMBNAIL_SUFFIX));
        self.large_thumbnail
            .as_ref()
            .expect("large thumbnail was just stored")
    }

    fn rendition(&self, suffix: &str) -> FileInfo {
        let basename = Path::new(&self.file.path)
            .file_stem()
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let filename = format!("{basename}{suffix}.{}", Self::IMAGE_EXTENSION);
        FileInfo {
            path: std::env::temp_dir()
                .join(filename)
                .to_string_lossy()
                .into_owned(),
            format: Self::IMAGE_FORMAT.to_owned(),
            content_type: Self::IMAGE_CONTENT_TYPE.to_owned(),
            ..FileInfo::default()
        }
    }
}
