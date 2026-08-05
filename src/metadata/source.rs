// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Channel and acquisition sources.

use chrono::{DateTime, Utc};
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use super::{Category, Scope};

/// A channel source associated with an entity type.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[model(unique(name = "source_code", fields(code)))]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Globally unique ASCII code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Source name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Ownership scope.
    #[model(index)]
    pub scope: Option<Scope>,
    /// Associated entity name.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub entity: String,
    /// Optional description.
    pub description: Option<String>,
    /// Optional category information.
    #[model(reference(target = Category, target_field = info), opaque)]
    pub category: Option<InfoWithEntity>,
    /// Whether this source is predefined.
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

impl Source {
    /// Returns whether the source has no identifying content.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id.is_none() && self.code.is_empty() && self.name.is_empty()
    }
}
