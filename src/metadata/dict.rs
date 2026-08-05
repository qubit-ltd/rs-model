// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Data-dictionary definitions.

use chrono::{DateTime, Utc};
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::commons::State;

use super::{Category, Scope};

/// A data dictionary without its entry collection.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[model(unique(name = "dict_code", fields(code)))]
#[serde(rename_all = "camelCase")]
pub struct Dict {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Globally unique stable code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Dictionary name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Ownership scope.
    #[model(index)]
    pub scope: Option<Scope>,
    /// Optional governing standards document.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub standard_doc: Option<String>,
    /// Optional code in the governing standard.
    #[model(index, text(min_chars = 1, max_chars = 64))]
    pub standard_code: Option<String>,
    /// Optional documentation URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    pub url: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Optional comment.
    pub comment: Option<String>,
    /// Optional category information.
    #[model(reference(target = Category, target_field = info), opaque)]
    pub category: Option<InfoWithEntity>,
    /// Lifecycle state.
    #[model(index)]
    #[serde(default)]
    pub state: State,
    /// Whether the dictionary is predefined.
    #[model(index)]
    pub predefined: bool,
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

impl Dict {
    /// Returns whether the dictionary has no identifying content.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id.is_none() && self.code.is_empty() && self.name.is_empty()
    }
}
