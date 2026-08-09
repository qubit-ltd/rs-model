// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Lightweight dictionary-entry projections used by common-domain records.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Compact identity and display data for a dictionary entry.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct DictEntryInfo {
    /// Platform-assigned identifier of the referenced entry.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Stable entry code in the referenced dictionary.
    pub code: String,

    /// Human-readable entry name.
    pub name: String,

    /// Identifier of the dictionary that owns the entry.
    #[model(identifier)]
    #[model(opaque)]
    pub dict_id: Id,

    /// Values substituted into numbered placeholders in code or name templates, in order.
    pub params: Vec<String>,

    /// UTC soft-deletion time, or `None` if the entry is still available.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
