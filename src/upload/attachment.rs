// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Persisted attachment metadata.

use chrono::{DateTime, Utc};
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::commons::{Owner, State};
use crate::person::UserInfo;
use crate::upload::{AttachmentType, Upload};

/// A categorized attachment owned by a domain object.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Attachment {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Object that owns this attachment.
    pub owner: Owner,

    /// Attachment type.
    pub r#type: AttachmentType,

    /// Optional attachment category.
    #[model(opaque)]
    pub category: Option<InfoWithEntity>,

    /// Zero-based ordering within the owner's attachment list.
    pub index: i32,

    /// Optional title.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub title: Option<String>,

    /// Optional description.
    pub description: Option<String>,

    /// Stored upload.
    pub upload: Upload,

    /// Lifecycle state.
    pub state: State,

    /// Whether this attachment is visible.
    pub visible: Option<bool>,

    /// Optional creator information.
    pub creator: Option<UserInfo>,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
