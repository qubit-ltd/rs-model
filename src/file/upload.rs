// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Upload records, original-file metadata, and generated image renditions.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use std::path::Path;
use uuid::Uuid;

use qubit_mixin::Emptyful;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::AttachmentType;
use super::FileInfo;
use super::UploadParams;

/// A received file together with metadata for its original and derived images.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct Upload {
    /// Database identifier; the default value denotes an upload not yet persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Original filename shown to users, or `None` when it was not supplied.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    #[redact(level = "secret")]
    pub original_filename: Option<String>,

    /// Attachment type inferred from the original file's MIME type.
    #[model(index)]
    pub r#type: AttachmentType,

    /// Storage metadata for the uploaded source file.
    #[redact(nested)]
    pub file: FileInfo,

    /// Screenshot rendition; `None` when this file kind has no screenshot.
    #[redact(nested)]
    pub screenshot: Option<FileInfo>,

    /// Small thumbnail rendition, or `None` when one was not generated.
    #[redact(nested)]
    pub small_thumbnail: Option<FileInfo>,

    /// Large thumbnail rendition, or `None` when one was not generated.
    #[redact(nested)]
    pub large_thumbnail: Option<FileInfo>,

    /// Digest algorithm name, or `None` when no source-file hash is tracked.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub hash_algorithm: Option<String>,

    /// Digest of the source file, or `None` when integrity verification is not used.
    #[model(text(min_chars = 1, max_chars = 512))]
    #[redact(level = "secret")]
    pub hash_value: Option<String>,

    /// UTC creation instant, or `None` until persistence assigns it.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the upload is retained.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Upload {
    /// Filename suffix used for screenshot renditions.
    pub const SCREENSHOT_SUFFIX: &'static str = "_screenshot";
    /// Filename suffix used for small-thumbnail renditions.
    pub const SMALL_THUMBNAIL_SUFFIX: &'static str = "_thumbnail_small";
    /// Filename suffix used for large-thumbnail renditions.
    pub const LARGE_THUMBNAIL_SUFFIX: &'static str = "_thumbnail_large";
    /// File extension assigned to generated image renditions.
    pub const IMAGE_EXTENSION: &'static str = ".jpg";
    /// Format label assigned to generated image renditions.
    pub const IMAGE_FORMAT: &'static str = "jpeg";
    /// MIME type assigned to generated image renditions.
    pub const IMAGE_CONTENT_TYPE: &'static str = "image/jpeg";

    /// Returns whether all fields use their default, empty, or absent representation.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id == Id::default()
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

    /// Creates upload metadata from a local file and caller-provided upload parameters.
    ///
    /// The caller supplies the original filename and mandatory MIME type through `params`.
    ///
    /// # Errors
    ///
    /// Returns `InvalidInput` if `params.content_type` is absent, or an I/O error if a
    /// relative `path` cannot be resolved against the current directory.
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

    /// Replaces original-file metadata using `path` and the supplied MIME type.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if a relative `path` cannot be resolved. A missing source
    /// receives a size of zero, matching the source model's `File.length()` behavior.
    pub fn set_file_info(&mut self, path: &Path, content_type: &str) -> std::io::Result<&FileInfo> {
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

    /// Sets the original MIME type and returns the corresponding inferred attachment type.
    pub fn set_content_type(&mut self, content_type: &str) -> AttachmentType {
        self.file.content_type = content_type.to_owned();
        self.r#type = AttachmentType::for_content_type(content_type);
        self.r#type
    }

    /// Creates and stores screenshot metadata with a unique path in the system temp directory.
    pub fn set_screenshot_info(&mut self) -> &FileInfo {
        self.screenshot = Some(self.rendition(Self::SCREENSHOT_SUFFIX));
        self.screenshot
            .as_ref()
            .expect("screenshot was just stored")
    }

    /// Creates and stores small-thumbnail metadata with a unique temporary path.
    pub fn set_small_thumbnail_info(&mut self) -> &FileInfo {
        self.small_thumbnail = Some(self.rendition(Self::SMALL_THUMBNAIL_SUFFIX));
        self.small_thumbnail
            .as_ref()
            .expect("small thumbnail was just stored")
    }

    /// Creates and stores large-thumbnail metadata with a unique temporary path.
    pub fn set_large_thumbnail_info(&mut self) -> &FileInfo {
        self.large_thumbnail = Some(self.rendition(Self::LARGE_THUMBNAIL_SUFFIX));
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
