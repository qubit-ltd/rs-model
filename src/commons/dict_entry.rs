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
use crate::metadata::Dict;
use crate::mixin::StatefulInfo;
/// A selectable entry in a named data dictionary.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[model(unique(name = "dict_entry_dict_code", fields(dict, code), ignore_case(code)))]
pub struct DictEntry {
    /// Platform-assigned identifier for this entry.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Stateful reference to the dictionary that contains this entry.
    #[model(reference(target = Dict, target_field = info))]
    pub dict: StatefulInfo,

    /// Stable code, unique within the owning dictionary.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Display name associated with the entry code.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional explanatory text for consumers of this entry.
    pub description: Option<String>,

    /// Optional administrator note that is separate from the description.
    pub comment: Option<String>,

    /// Optional parent entry for a hierarchical dictionary; `None` marks a root entry.
    #[model(reference(target = DictEntry, target_field = info), opaque)]
    pub parent: Option<Box<DictEntryInfo>>,

    /// Creation time in UTC, stored with second precision.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Most recent modification time in UTC, or `None` if unchanged.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion time, or `None` when the entry is not deleted.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
