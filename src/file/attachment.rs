// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted attachments associated with aggregate-root properties.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::AttachmentType;
use super::Upload;
use crate::commons::State;
use crate::metadata::AggregateRef;
use crate::metadata::Category;

/// A stored upload attached to a categorized property of an aggregate root.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display)]
#[serde(default)]
pub struct Attachment {
    /// Database identifier; the default value denotes an attachment not yet persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Aggregate and property that own this attachment, or `None` before association.
    #[model(index)]
    #[redact(nested)]
    pub aggregate_ref: Option<AggregateRef>,

    /// Classification used to choose attachment-specific handling and presentation.
    #[model(index)]
    pub r#type: AttachmentType,

    /// Category reference, or `None` when the owner has not classified the attachment.
    #[model(reference(target = Category, target_field = info), opaque)]
    pub category: Option<InfoWithEntity>,

    /// Zero-based order within the owning aggregate property's attachment collection.
    #[model(index)]
    #[serde(default)]
    pub index: i32,

    /// User-facing title, or `None` when the attachment has no title.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    #[redact(level = "secret")]
    pub title: Option<String>,

    /// Additional attachment description, or `None` when none was entered.
    pub description: Option<String>,

    /// Upload record that provides the original file and generated renditions.
    #[model(reference(target = Upload, target_field = id))]
    #[redact(nested)]
    pub upload: Upload,

    /// Lifecycle state that governs use of the attachment.
    #[model(index)]
    pub state: State,

    /// Whether the attachment is visible to permitted consumers; defaults to `true`.
    #[model(index)]
    #[serde(default = "default_visible")]
    pub visible: bool,

    /// UTC creation instant, or `None` until persistence assigns it.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,

    /// UTC instant of the latest update, or `None` when unchanged.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the attachment is retained.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

/// Returns the source default for attachment visibility.
const fn default_visible() -> bool {
    true
}

impl Attachment {
    /// Creates a normal, visible attachment from `upload`, using its filename and type.
    #[must_use]
    pub fn create(upload: Upload) -> Self {
        Self {
            title: upload.original_filename.clone(),
            r#type: upload.r#type,
            upload,
            ..Self::default()
        }
    }

    /// Returns the original upload's storage path.
    #[must_use]
    pub fn file_path(&self) -> Option<&str> {
        Some(&self.upload.file.path)
    }

    /// Returns the screenshot rendition path, or `None` when no screenshot exists.
    #[must_use]
    pub fn screenshot_path(&self) -> Option<&str> {
        self.upload
            .screenshot
            .as_ref()
            .map(|file| file.path.as_str())
    }

    /// Returns the large-thumbnail path, or `None` when that rendition is absent.
    #[must_use]
    pub fn large_thumbnail_path(&self) -> Option<&str> {
        self.upload
            .large_thumbnail
            .as_ref()
            .map(|file| file.path.as_str())
    }

    /// Returns the small-thumbnail path, or `None` when that rendition is absent.
    #[must_use]
    pub fn small_thumbnail_path(&self) -> Option<&str> {
        self.upload
            .small_thumbnail
            .as_ref()
            .map(|file| file.path.as_str())
    }
}

/// Borrowed JSON-wire projection for an [`Attachment`].
#[derive(Serialize)]
struct AttachmentWire<'a> {
    id: Id,
    aggregate_ref: Option<&'a AggregateRef>,

    #[serde(rename = "type")]
    r#type: &'a AttachmentType,
    category: Option<&'a InfoWithEntity>,
    index: i32,
    title: Option<&'a str>,
    description: Option<&'a str>,
    upload: &'a Upload,
    state: &'a State,
    visible: bool,
    create_time: Option<&'a DateTime<Utc>>,
    modify_time: Option<&'a DateTime<Utc>>,
    delete_time: Option<&'a DateTime<Utc>>,
    file_path: &'a str,
    screenshot_path: Option<&'a str>,
    large_thumbnail_path: Option<&'a str>,
    small_thumbnail_path: Option<&'a str>,
}

impl Serialize for Attachment {
    /// Serializes stored fields and the computed original, screenshot, and thumbnail paths.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        AttachmentWire {
            id: self.id,
            aggregate_ref: self.aggregate_ref.as_ref(),
            r#type: &self.r#type,
            category: self.category.as_ref(),
            index: self.index,
            title: self.title.as_deref(),
            description: self.description.as_deref(),
            upload: &self.upload,
            state: &self.state,
            visible: self.visible,
            create_time: self.create_time.as_ref(),
            modify_time: self.modify_time.as_ref(),
            delete_time: self.delete_time.as_ref(),
            file_path: &self.upload.file.path,
            screenshot_path: self.screenshot_path(),
            large_thumbnail_path: self.large_thumbnail_path(),
            small_thumbnail_path: self.small_thumbnail_path(),
        }
        .serialize(serializer)
    }
}

impl Default for Attachment {
    fn default() -> Self {
        Self {
            id: Id::default(),
            aggregate_ref: None,
            r#type: AttachmentType::default(),
            category: None,
            index: 0,
            title: None,
            description: None,
            upload: Upload::default(),
            state: State::Normal,
            visible: true,
            create_time: None,
            modify_time: None,
            delete_time: None,
        }
    }
}
