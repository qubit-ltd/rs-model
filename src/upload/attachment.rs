// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Attachment records owned by a domain object.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;

use crate::commons::Category;
use crate::commons::Owner;
use crate::commons::State;
use crate::person::User;
use crate::person::UserInfo;
use crate::upload::AttachmentType;
use crate::upload::Upload;

/// A classified upload attached to an owning domain object.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Attachment {
    /// Database identifier; the default value denotes an attachment not yet persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Domain object that owns this attachment.
    #[model(index)]
    pub owner: Owner,

    /// Content classification of the attachment.
    #[model(index)]
    pub r#type: AttachmentType,

    /// Category reference, or `None` when the attachment is uncategorized.
    #[model(reference(target = Category, target_field = info, must_exist = true), opaque)]
    pub category: Option<InfoWithEntity>,

    /// Zero-based position in the owner's attachment list.
    pub index: i32,

    /// User-facing title, or `None` when no title is set.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub title: Option<String>,

    /// Additional descriptive text, or `None` when absent.
    pub description: Option<String>,

    /// Upload record containing the stored original file and its renditions.
    #[model(reference(target = Upload, target_field = id, must_exist = true))]
    pub upload: Upload,

    /// Lifecycle state governing this attachment.
    #[model(index)]
    pub state: State,

    /// Visibility flag, or `None` when visibility is unspecified.
    pub visible: Option<bool>,

    /// Creator reference, or `None` when the creator was not captured.
    #[model(reference(target = User, target_field = info, must_exist = true))]
    pub creator: Option<UserInfo>,

    /// UTC creation instant, rounded to seconds.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the latest update, or `None` when unchanged.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while retained.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
