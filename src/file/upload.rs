// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted upload metadata and generated renditions.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use uuid::Uuid;

use qubit_mixin::Emptyful;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::AttachmentType;
use super::FileInfo;
use super::UploadParams;

/// A received file and its generated image renditions.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize,
)]
#[serde(default)]
pub struct Upload {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Optional user-facing original filename.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_filename: Option<String>,

    /// Type inferred for the uploaded file.
    #[model(index)]
    pub r#type: AttachmentType,

    /// Original file storage metadata.
    #[redact(nested)]
    pub file: FileInfo,

    /// Optional screenshot rendition.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot: Option<FileInfo>,

    /// Optional small-thumbnail rendition.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_thumbnail: Option<FileInfo>,

    /// Optional large-thumbnail rendition.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_thumbnail: Option<FileInfo>,

    /// Optional hash algorithm name.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_algorithm: Option<String>,

    /// Optional content hash value.
    #[model(text(min_chars = 1, max_chars = 512))]
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_value: Option<String>,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Upload {
    /// Suffix appended to screenshot renditions.
    pub const SCREENSHOT_SUFFIX: &'static str = "_screenshot";
    /// Suffix appended to small thumbnails.
    pub const SMALL_THUMBNAIL_SUFFIX: &'static str = "_thumbnail_small";
    /// Suffix appended to large thumbnails.
    pub const LARGE_THUMBNAIL_SUFFIX: &'static str = "_thumbnail_large";
    /// Generated image extension.
    pub const IMAGE_EXTENSION: &'static str = ".jpg";
    /// Generated image format.
    pub const IMAGE_FORMAT: &'static str = "jpeg";
    /// Generated image MIME type.
    pub const IMAGE_CONTENT_TYPE: &'static str = "image/jpeg";

    /// Returns whether every source property is empty or at its Rust null
    /// representation.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.original_filename.as_ref().is_none_or(String::is_empty)
            && self.r#type == AttachmentType::Document
            && self.file.is_empty()
            && self.screenshot.as_ref().is_none_or(FileInfo::is_empty)
            && self.small_thumbnail.as_ref().is_none_or(FileInfo::is_empty)
            && self.large_thumbnail.as_ref().is_none_or(FileInfo::is_empty)
            && self.hash_algorithm.as_ref().is_none_or(String::is_empty)
            && self.hash_value.as_ref().is_none_or(String::is_empty)
            && self.create_time.is_none()
            && self.delete_time.is_none()
    }

    /// Creates upload metadata for an existing local file.
    ///
    /// The original filename and content type are taken directly from the
    /// upload parameters.
    ///
    /// # Errors
    ///
    /// Returns [`std::io::ErrorKind::InvalidInput`] when the required content
    /// type is absent, or an I/O error when a relative path cannot be made
    /// absolute.
    pub fn create(path: &Path, params: &UploadParams) -> std::io::Result<Self> {
        let content_type = params.content_type.as_deref().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "upload content type is required",
            )
        })?;
        let mut upload = Self {
            original_filename: params.filename.clone(),
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
    /// Returns an I/O error when a relative path cannot be made absolute.
    /// A nonexistent source path has size zero, matching `File.length()`.
    pub fn set_file_info(
        &mut self,
        path: &Path,
        content_type: &str,
    ) -> std::io::Result<&FileInfo> {
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };
        self.file = FileInfo {
            path: absolute_path.to_string_lossy().into_owned(),
            content_type: content_type.to_owned(),
            size: std::fs::metadata(path)
                .map(|metadata| metadata.len())
                .unwrap_or(0)
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
        self.small_thumbnail =
            Some(self.rendition(Self::SMALL_THUMBNAIL_SUFFIX));
        self.small_thumbnail
            .as_ref()
            .expect("small thumbnail was just stored")
    }

    /// Creates and stores large-thumbnail metadata in the temporary directory.
    pub fn set_large_thumbnail_info(&mut self) -> &FileInfo {
        self.large_thumbnail =
            Some(self.rendition(Self::LARGE_THUMBNAIL_SUFFIX));
        self.large_thumbnail
            .as_ref()
            .expect("large thumbnail was just stored")
    }

    /// Builds unique temporary metadata for an image rendition.
    fn rendition(&self, suffix: &str) -> FileInfo {
        let basename = Path::new(&self.file.path)
            .file_stem()
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let filename = format!(
            "{basename}{suffix}-{}{}",
            Uuid::new_v4(),
            Self::IMAGE_EXTENSION
        );
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

impl Emptyful for Upload {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}
