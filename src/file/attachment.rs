// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted attachment metadata.

use chrono::{
    DateTime,
    Utc,
};
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
    Serializer,
};

use crate::{
    commons::State,
    metadata::{
        AggregateRef,
        Category,
    },
};

use super::{
    AttachmentType,
    Upload,
};

/// A categorized attachment belonging to an aggregate root.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact)]
#[serde(default)]
pub struct Attachment {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Owning aggregate-root reference.
    #[model(index)]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_ref: Option<AggregateRef>,
    /// Attachment classification.
    #[model(index)]
    pub r#type: AttachmentType,
    /// Optional category information.
    #[model(reference(target = Category, target_field = info), opaque)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<InfoWithEntity>,
    /// Zero-based order within the aggregate property.
    #[model(index)]
    #[serde(default)]
    pub index: i32,
    /// Optional title.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Stored upload.
    #[model(reference(target = Upload, target_field = id))]
    #[redact(nested)]
    pub upload: Upload,
    /// Lifecycle state.
    #[model(index)]
    pub state: State,
    /// Whether the attachment is visible.
    #[model(index)]
    #[serde(default = "default_visible")]
    pub visible: bool,
    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,
    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<DateTime<Utc>>,
}

/// Returns the source default for attachment visibility.
const fn default_visible() -> bool {
    true
}

impl Attachment {
    /// Creates an attachment from an upload using source defaults.
    #[must_use]
    pub fn create(upload: Upload) -> Self {
        Self {
            title: upload.original_filename.clone(),
            r#type: upload.r#type,
            upload,
            ..Self::default()
        }
    }

    /// Returns the original file path.
    #[must_use]
    pub fn file_path(&self) -> Option<&str> {
        Some(&self.upload.file.path)
    }

    /// Returns the screenshot path when present.
    #[must_use]
    pub fn screenshot_path(&self) -> Option<&str> {
        self.upload
            .screenshot
            .as_ref()
            .map(|file| file.path.as_str())
    }

    /// Returns the large-thumbnail path when present.
    #[must_use]
    pub fn large_thumbnail_path(&self) -> Option<&str> {
        self.upload
            .large_thumbnail
            .as_ref()
            .map(|file| file.path.as_str())
    }

    /// Returns the small-thumbnail path when present.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregate_ref: Option<&'a AggregateRef>,
    #[serde(rename = "type")]
    r#type: &'a AttachmentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<&'a InfoWithEntity>,
    index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    upload: &'a Upload,
    state: &'a State,
    visible: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<&'a DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modify_time: Option<&'a DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_time: Option<&'a DateTime<Utc>>,
    file_path: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    screenshot_path: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    large_thumbnail_path: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    small_thumbnail_path: Option<&'a str>,
}

impl Serialize for Attachment {
    /// Serializes persisted fields plus source computed path properties.
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
            id: None,
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
