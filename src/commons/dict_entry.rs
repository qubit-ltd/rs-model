// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Complete records for entries in legacy common dictionaries.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use super::DictEntryInfo;
use crate::mixin::StatefulInfo;
/// A selectable entry in a named data dictionary.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DictEntry {
    /// Platform-assigned identifier for this entry.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Stateful reference to the dictionary that contains this entry.
    pub dict: StatefulInfo,

    /// Stable code, unique within the owning dictionary.
    pub code: String,

    /// Display name associated with the entry code.
    pub name: String,

    /// Optional explanatory text for consumers of this entry.
    pub description: Option<String>,

    /// Optional administrator note that is separate from the description.
    pub comment: Option<String>,

    /// Optional parent entry for a hierarchical dictionary; `None` marks a root entry.
    #[model(opaque)]
    pub parent: Option<Box<DictEntryInfo>>,

    /// Creation time in UTC, stored with second precision.
    #[model(time(precision=second,normalization=utc))]
    pub create_time: DateTime<Utc>,

    /// Most recent modification time in UTC, or `None` if unchanged.
    #[model(time(precision=second,normalization=utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion time, or `None` when the entry is not deleted.
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
