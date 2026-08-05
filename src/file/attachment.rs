// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Persisted attachment metadata.

use chrono::{DateTime, Utc};
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::{
    commons::State,
    metadata::{AggregateRef, Category},
};

use super::{AttachmentType, Upload};

/// A categorized attachment belonging to an aggregate root.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Owning aggregate-root reference.
    #[model(index)]
    #[redact(nested)]
    pub aggregate_ref: Option<AggregateRef>,
    /// Attachment classification.
    #[model(index)]
    pub r#type: AttachmentType,
    /// Optional category information.
    #[model(reference(target = Category, target_field = info), opaque)]
    pub category: Option<InfoWithEntity>,
    /// Zero-based order within the aggregate property.
    #[model(index)]
    pub index: i32,
    /// Optional title.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    #[redact(level = "secret")]
    pub title: Option<String>,
    /// Optional description.
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
    pub visible: bool,
    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,
    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
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
        nonempty_path(&self.upload.file.path)
    }

    /// Returns the screenshot path when present.
    #[must_use]
    pub fn screenshot_path(&self) -> Option<&str> {
        self.upload
            .screenshot
            .as_ref()
            .and_then(|file| nonempty_path(&file.path))
    }

    /// Returns the large-thumbnail path when present.
    #[must_use]
    pub fn large_thumbnail_path(&self) -> Option<&str> {
        self.upload
            .large_thumbnail
            .as_ref()
            .and_then(|file| nonempty_path(&file.path))
    }

    /// Returns the small-thumbnail path when present.
    #[must_use]
    pub fn small_thumbnail_path(&self) -> Option<&str> {
        self.upload
            .small_thumbnail
            .as_ref()
            .and_then(|file| nonempty_path(&file.path))
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

fn nonempty_path(path: &str) -> Option<&str> {
    (!path.is_empty()).then_some(path)
}
