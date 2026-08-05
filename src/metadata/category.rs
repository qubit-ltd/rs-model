// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Hierarchical metadata categories.

use chrono::{DateTime, Utc};
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use super::Scope;

/// A hierarchical category associated with an entity type.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[model(
    unique(name = "category_code", fields(code)),
    unique(name = "category_entity_name", fields(entity, name))
)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Globally unique ASCII code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Name unique within the associated entity.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Ownership scope.
    #[model(index)]
    pub scope: Option<Scope>,
    /// Associated entity name.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub entity: String,
    /// Optional icon URI or key.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Query-computed hierarchical title.
    #[model(text(min_chars = 1, max_chars = 4096))]
    pub title: Option<String>,
    /// Optional parent-category information.
    #[model(reference(target = Category, target_field = info), index, opaque)]
    pub parent: Option<InfoWithEntity>,
    /// Whether the category is predefined.
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

impl Category {
    /// Separator used to join names into a hierarchical title.
    pub const TITLE_JOINER: &'static str = " - ";

    /// Returns whether the category has no identifying content.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id.is_none() && self.code.is_empty() && self.name.is_empty()
    }
}
