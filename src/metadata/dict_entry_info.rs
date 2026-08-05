// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Compact dictionary-entry information.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use super::{Dict, dict_entry::format_with_params};

/// Compact information for a dictionary entry.
#[derive(Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DictEntryInfo {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Entry code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Entry name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Owning dictionary identifier.
    #[model(reference(target = Dict, target_field = id))]
    pub dict_id: Option<i64>,
    /// Values substituted into numbered placeholders.
    #[model(sequence(max_items = 5))]
    pub params: Vec<String>,
    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl DictEntryInfo {
    /// Creates an info value unless every supplied identity field is absent.
    #[must_use]
    pub fn create(id: Option<i64>, code: Option<&str>, name: Option<&str>) -> Option<Self> {
        if id.is_none() && code.is_none() && name.is_none() {
            None
        } else {
            Some(Self {
                id,
                code: code.unwrap_or_default().to_owned(),
                name: name.unwrap_or_default().to_owned(),
                ..Self::default()
            })
        }
    }

    /// Returns the code with numbered placeholders substituted.
    #[must_use]
    pub fn display_code(&self) -> String {
        format_with_params(&self.code, &self.params)
    }

    /// Returns the name with numbered placeholders substituted.
    #[must_use]
    pub fn display_name(&self) -> String {
        format_with_params(&self.name, &self.params)
    }

    /// Returns whether all identifying fields are empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id.is_none() && self.code.is_empty() && self.name.is_empty()
    }
}
