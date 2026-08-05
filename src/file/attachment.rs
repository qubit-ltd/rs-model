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
    ser::SerializeStruct,
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

impl Serialize for Attachment {
    /// Serializes persisted fields plus source computed path properties.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Attachment", 17)?;
        if let Some(id) = self.id {
            state.serialize_field("id", &id)?;
        }
        if let Some(aggregate_ref) = &self.aggregate_ref {
            state.serialize_field("aggregate_ref", aggregate_ref)?;
        }
        state.serialize_field("type", &self.r#type)?;
        if let Some(category) = &self.category {
            state.serialize_field("category", category)?;
        }
        state.serialize_field("index", &self.index)?;
        if let Some(title) = &self.title {
            state.serialize_field("title", title)?;
        }
        if let Some(description) = &self.description {
            state.serialize_field("description", description)?;
        }
        state.serialize_field("upload", &self.upload)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("visible", &self.visible)?;
        if let Some(create_time) = self.create_time {
            state.serialize_field("create_time", &create_time)?;
        }
        if let Some(modify_time) = self.modify_time {
            state.serialize_field("modify_time", &modify_time)?;
        }
        if let Some(delete_time) = self.delete_time {
            state.serialize_field("delete_time", &delete_time)?;
        }
        state.serialize_field("file_path", &self.upload.file.path)?;
        if let Some(path) = self.screenshot_path() {
            state.serialize_field("screenshot_path", path)?;
        }
        if let Some(path) = self.large_thumbnail_path() {
            state.serialize_field("large_thumbnail_path", path)?;
        }
        if let Some(path) = self.small_thumbnail_path() {
            state.serialize_field("small_thumbnail_path", path)?;
        }
        state.end()
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
